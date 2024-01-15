use std::{collections::HashMap, sync::Arc, ops::Add};

use actix_web::web;
use actix_web_lab::__reexports::tokio::task::JoinSet;
use chrono::{Days, NaiveDate};
use log::debug;
use reqwest::{Client, Response};

use super::utils::{
    self, day_of_week, DetailedSubject, FormattedLesson, Holidays, Klasse, LoginResults, PeriodObject, Schoolyear, Substitution, TimegridUnits, TimetableParameter, UntisArrayResponse
};
use crate::{
    api_wrapper::utils::UntisResponse, models::{model::DBConnection, teacher_model::Teacher}, prelude::Error
};

#[derive(Clone)]
pub struct UntisClient {
    pub person_type: u16,
    pub person_id: u16,
    id: String,
    school: String,
    subdomain: String,
    client: Client,
    jsessionid: String,
    ids: HashMap<String, u16>,
    db: web::Data<DBConnection>,
}

#[allow(dead_code)]
impl UntisClient {
    async fn request(&self, params: utils::Parameter, method: String) -> Result<Response, Error> {
        let body = utils::UntisBody {
            school: self.school.clone(),
            id: self.id.clone(),
            method,
            params,
            jsonrpc: "2.0".to_string(),
        };

        let response = self
            .client
            .post(format!("https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}", self.subdomain, self.school))
            .body(serde_json::to_string(&body).map_err(|_| Error::UntisError("Error parsing request Body (43)".into()))?)
            .header("Cookie", "JSESSIONID=".to_owned() + &self.jsessionid)
            .send()
            .await
            .map_err(Error::Reqwest)?;

        debug!("{:?}", response);

        Ok(response)
    }

    pub async fn init(
        user: String, password: String, id: String, school: String, subdomain: String, db: web::Data<DBConnection>,
    ) -> Result<Self, Error> {
        let mut untis_client = Self {
            person_type: 0,
            person_id: 0,
            id,
            school,
            subdomain,
            client: Client::new(),
            jsessionid: "".to_string(),
            ids: HashMap::new(),
            db,
        };

        untis_client.login(user, password).await.map_err(|err| Error::UntisError(err.to_string() + " 69".into()))?;
        untis_client.ids = untis_client.get_ids().await.map_err(|err| Error::UntisError(err.to_string() + " 70".into()))?;

        Ok(untis_client)
    }

    pub async fn unsafe_init(
        jsessionid: String, person_id: u16, person_type: u16, id: String, school: String, subdomain: String,
        db: web::Data<DBConnection>,
    ) -> Result<Self, Error> {
        let client = Client::new();

        let mut untis_client = Self {
            person_type,
            person_id,
            id,
            school,
            subdomain,
            client,
            jsessionid,
            ids: HashMap::new(),
            db,
        };

        untis_client.ids = untis_client.get_ids().await?;

        Ok(untis_client)
    }

    async fn login(&mut self, user: String, password: String) -> Result<bool, Box<dyn std::error::Error>> {
        let params = utils::Parameter::AuthParameter(utils::AuthParameter {
            user,
            password,
            client: self.id.clone(),
        });

        let response = self.request(params, "authenticate".to_string()).await?;

        let text = response.text().await?;
        let json: UntisResponse<LoginResults> = serde_json::from_str(&text)?;

        self.jsessionid = json.result.session_id;
        self.person_id = json.result.person_id;
        self.person_type = json.result.person_type;

        Ok(true)
    }

    async fn get_ids(&mut self) -> Result<HashMap<String, u16>, Error> {
        let klassen: Vec<Klasse> = self.get_klassen().await.map_err(|err| Error::UntisError(err.to_string() + " 118".into()))?;

        let ef_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "EF")
            .ok_or("Couldn't find EF")
            .map_err(|err| Error::UntisError(err.to_string() + " 125".into()))?;
        let q1_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "Q1")
            .ok_or("Couldn't find Q1")
            .map_err(|err| Error::UntisError(err.to_string() + " 131".into()))?;
        let q2_id = klassen
            .into_iter()
            .find(|klasse| klasse.name == "Q2")
            .ok_or("Couldn't find Q2")
            .map_err(|err| Error::UntisError(err.to_string() + " 136".into()))?;

        Ok(HashMap::from([
            ("EF".into(), ef_id.id),
            ("Q1".into(), q1_id.id),
            ("Q2".into(), q2_id.id),
        ]))
    }

    pub fn logout(&mut self) -> Result<bool, Error> {
        let _reponse = self.request(utils::Parameter::Null(), "logout".to_string());

        Ok(true)
    }

    pub async fn get_timetable(&self, parameter: TimetableParameter) -> Result<Vec<FormattedLesson>, Error> {
        let response = self
            .request(utils::Parameter::TimetableParameter(parameter.clone()), "getTimetable".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 155".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 157".into()))?;
        let json: UntisArrayResponse<PeriodObject> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 158".into()))?;

        let mut timetable = json.result;

        let mut holidays = self
            .get_period_holidays(
                parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 164".into()))?,
                parameter.options.end_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 165".into()))?,
            )
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 168".into()))?;

        timetable.append(&mut holidays);

        self.format_lessons(timetable, parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 172".into()))?)
            .await
    }

    pub async fn get_period_holidays(&self, start_date: u32, end_date: u32) -> Result<Vec<PeriodObject>, Error> {
        let all_holidays = self.get_holidays().await.map_err(|err| Error::UntisError(err.to_string() + " 172".into()))?;

        let holidays = all_holidays.iter().filter(|&holiday| {
            holiday.start_date <= i64::from(start_date) && holiday.end_date >= i64::from(start_date)
                || holiday.start_date <= i64::from(end_date) && holiday.end_date >= i64::from(start_date)
        });

        let mut period_holidays: Vec<PeriodObject> = vec![];

        for holiday in holidays {
            let start = NaiveDate::parse_from_str(&start_date.to_string(), "%Y%m%d").map_err(|err| Error::UntisError(err.to_string() + " 187".into()))?;
            let end = NaiveDate::parse_from_str(&end_date.to_string(), "%Y%m%d").map_err(|err| Error::UntisError(err.to_string() + " 188".into()))?;

            let length = end - start;

            for i in 0..=length.num_days() {
                if let Some(date) = start.checked_add_days(Days::new(i as u64)) {
                    if NaiveDate::parse_from_str(&holiday.start_date.to_string(), "%Y%m%d")
                        .map_err(|err| Error::UntisError(err.to_string() + " 195".into()))?
                        > date
                    {
                        continue;
                    }
                    if NaiveDate::parse_from_str(&holiday.end_date.to_string(), "%Y%m%d")
                        .map_err(|err| Error::UntisError(err.to_string() + " 201".into()))?
                        < date
                    {
                        break;
                    }
                    period_holidays.push(PeriodObject {
                        id: 1,
                        date: date.format("%Y%m%d").to_string().parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 208".into()))?,
                        start_time: 755,
                        end_time: 1625,
                        kl: vec![],
                        te: vec![],
                        su: vec![],
                        ro: vec![],
                        activity_type: None,
                        subst_text: None,
                        lstext: Some(match holiday.longname.clone() {
                            Some(longname) => longname,
                            None => holiday.name.clone(),
                        }),
                        code: None,
                        sg: None,
                    })
                }
            }
        }
        Ok(period_holidays)
    }

    async fn format_lessons(
        &self, mut lessons: Vec<PeriodObject>, start_date: u32,
    ) -> Result<Vec<FormattedLesson>, Error> {
        let mut formatted: Vec<FormattedLesson> = vec![];

        lessons.sort_unstable_by_key(|les| les.date);
        let mut days: Vec<Vec<PeriodObject>> = vec![];

        let mut date = start_date;
        let mut day: Vec<PeriodObject> = vec![];

        for l in lessons {
            if l.date != date {
                day.sort_unstable_by_key(|ele| ele.start_time);
                let d = &day;
                days.push(d.to_owned());
                let new_date = &l.date;
                date = new_date.to_owned();
                day = vec![l];
            } else {
                day.push(l)
            }
        }
        day.sort_unstable_by_key(|ele| ele.start_time);
        let d = &day;
        days.push(d.to_owned());

        let mut skip: HashMap<u16, u8> = HashMap::new();
        let timegrid = self.get_timegrid_units().await.map_err(|err| Error::UntisError(err.to_string() + " 258".into()))?;

        for d in days {
            let clone = d.clone();
            for lesson in clone {
                let is_exam: bool = match lesson.sg {
                    Some(sg) => sg.starts_with("EXAM"),
                    None => false,
                };
                if !lesson.su.is_empty() && skip.contains_key(&lesson.su[0].id) && skip[&lesson.su[0].id] > 0 {
                    skip.entry(lesson.su[0].id).and_modify(|skips| *skips -= 1);
                    if skip[&lesson.su[0].id] == 0 {
                        skip.remove(&lesson.su[0].id);
                    }
                    continue;
                }
                let day = day_of_week(lesson.date);
                let start = timegrid[usize::try_from(day).map_err(|err| Error::UntisError(err.to_string() + " 275".into()))?]
                    .time_units
                    .iter()
                    .position(|unit| unit.start_time == lesson.start_time)
                    .get_or_insert(0)
                    .add(1);
                let mut subject = "".to_string();
                let mut subject_short = "".to_string();

                if !lesson.su.is_empty() {
                    subject = lesson.su[0].name.to_owned();
                    subject_short = lesson.su[0].name.to_owned();
                    subject_short = subject_short.split(' ').collect::<Vec<&str>>()[0].to_owned();
                }

                match lesson.code.clone() {
                    Some(code) => {
                        if code == "irregular" {
                            subject = match lesson.subst_text.clone() {
                                Some(text) => text,
                                None => "Entfällt".to_string(),
                            }
                        }
                    }
                    None => {
                        if lesson.su.is_empty() {
                            if lesson.activity_type.is_none() {
                                match lesson.lstext {
                                    Some(text) => {
                                        subject = text.clone();
                                        let mut split: Vec<&str> = text.split_whitespace().collect();
                                        if split.len() >= 2 {
                                            split.remove(0);
                                            split.remove(0);
                                        }
                                        subject_short = split.join(" ");
                                    }
                                    None => {
                                        subject = "N/A".to_string();
                                        subject_short = "N/A".to_string();
                                    }
                                }
                            } else {
                                match lesson.lstext {
                                    Some(text) => {
                                        subject = text.clone();
                                        subject_short = text.clone();
                                    }
                                    None => {
                                        subject = "N/A".to_string();
                                        subject_short = "N/A".to_string();
                                    }
                                }
                            }
                        }
                    }
                }
                let mut substituted = false;

                if is_exam {
                    subject = "Prüfung".to_string() + &subject;
                }

                let teacher = if !lesson.te.is_empty() {
                    match lesson.te[0].orgname.clone() {
                        Some(orgname) => {
                            substituted = true;
                            orgname
                        }
                        None => lesson.te[0].name.to_owned(),
                    }
                } else {
                    "".to_string()
                };

                let room = if !lesson.ro.is_empty() {
                    match lesson.ro[0].orgname.clone() {
                        Some(orgname) => {
                            substituted = true;
                            orgname
                        }
                        None => lesson.ro[0].name.to_owned(),
                    }
                } else {
                    "".to_string()
                };

                if lesson.code == Some("cancelled".to_string()) || lesson.subst_text.is_some() {
                    substituted = true;
                }

                let mut formatted_lesson = FormattedLesson {
                    teacher,
                    is_lb: false,
                    start: u8::try_from(start).map_err(|err| Error::UntisError(err.to_string() + " 369".into()))?,
                    length: if !lesson.su.is_empty()
                        && d.iter().any(|les| {
                            !les.su.is_empty()
                                && les.su[0].id == lesson.su[0].id
                                && (les.start_time == lesson.end_time
                                    || les.start_time == lesson.end_time + 5
                                    || les.start_time == lesson.end_time + 20) // !! Could break !!
                        }) {
                        if d.iter().any(|les| {
                            !les.su.is_empty()
                                && les.su[0].id == lesson.su[0].id
                                && (les.end_time == lesson.start_time
                                    || les.end_time == lesson.start_time - 5
                                    || les.start_time == lesson.end_time + 20)
                        }) {
                            3
                        } else {
                            2
                        }
                    } else if (lesson.end_time - lesson.start_time) > 85 {
                        if ((((lesson.end_time - lesson.start_time) / 85) as f32).floor() as u8) > 10{
                            10
                        }
                        else{
                            (((lesson.end_time - lesson.start_time) / 85) as f32).floor() as u8
                        }
                    } else {
                        1
                    },
                    day,
                    subject,
                    subject_short,
                    room,
                    substitution: if substituted {
                        Some(Substitution {
                            teacher: if !lesson.te.is_empty() {
                                if lesson.te[0].orgname.is_some() {
                                    Some(lesson.te[0].name.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            },
                            room: if !lesson.ro.is_empty() {
                                if lesson.ro[0].orgname.is_some() {
                                    Some(lesson.ro[0].name.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            },
                            substitution_text: lesson.subst_text.clone(),
                            cancelled: match lesson.code {
                                Some(code) => {
                                    code == *"cancelled"
                                        || match lesson.subst_text {
                                            Some(text) => text == *"Vtr. ohne Lehrer",
                                            None => false,
                                        }
                                }
                                None => false,
                            },
                        })
                    } else {
                        None
                    },
                };
                formatted_lesson.is_lb = formatted_lesson.length == 1 && !is_exam;
                if formatted_lesson.length > 1 && !lesson.su.is_empty() {
                    skip.insert(lesson.su[0].id, formatted_lesson.length - 1);
                }
                formatted.push(formatted_lesson);
            }
        }

        Ok(formatted)
    }

    pub async fn get_lernbueros(&self, mut parameter: TimetableParameter) -> Result<Vec<FormattedLesson>, Error> {
        let mut all_lbs: Vec<FormattedLesson> = vec![];
        let mut future_lessons = JoinSet::new();

        let ef_id = self.ids.get(&"EF".to_string()).ok_or("Couldn't find field EF").map_err(|err| Error::UntisError(err.to_string() + " 454".into()))?;
        let q1_id = self.ids.get(&"Q1".to_string()).ok_or("Couldn't find field Q1").map_err(|err| Error::UntisError(err.to_string() + " 455".into()))?;
        let q2_id = self.ids.get(&"Q2".to_string()).ok_or("Couldn't find field Q2").map_err(|err| Error::UntisError(err.to_string() + " 456".into()))?;

        parameter.options.element.r#type = 1;

        let mut ef_parameter = parameter.clone();
        let mut q1_parameter = parameter.clone();
        let mut q2_parameter = parameter.clone();

        ef_parameter.options.element.id = ef_id.to_owned();
        q1_parameter.options.element.id = q1_id.to_owned();
        q2_parameter.options.element.id = q2_id.to_owned();

        let ef_client = Arc::new(self.clone());
        future_lessons.spawn(async move { ef_client.clone().get_timetable(ef_parameter).await });
        let q1_client = Arc::new(self.clone());
        future_lessons.spawn(async move { q1_client.clone().get_timetable(q1_parameter).await });
        let q2_client = Arc::new(self.clone());
        future_lessons.spawn(async move { q2_client.clone().get_timetable(q2_parameter).await });

        let mut lessons: Vec<Vec<FormattedLesson>> = vec![];

        while let Some(res) = future_lessons.join_next().await {
            lessons.push(res.map_err(|err| Error::UntisError(err.to_string()))?.map_err(|err| Error::UntisError(err.to_string() + " 478".into()))?)
        }

        all_lbs.append(
            &mut lessons[0].clone().into_iter().filter(|lesson| lesson.is_lb).collect::<Vec<FormattedLesson>>(),
        );
        all_lbs.append(
            &mut lessons[1].clone().into_iter().filter(|lesson| lesson.is_lb).collect::<Vec<FormattedLesson>>(),
        );
        all_lbs.append(
            &mut lessons[2].clone().into_iter().filter(|lesson| lesson.is_lb).collect::<Vec<FormattedLesson>>(),
        );

        let mut additional_lbs: Vec<FormattedLesson> = vec![];

        for lb in all_lbs.clone() {
            let mut new_room = "".to_string();
            if let Some(sub) = lb.clone().substitution {
                if sub.clone().cancelled {
                    continue;
                }
                if let Some(r) = sub.room {
                    new_room = r;
                }
            }
            let pot_teacher = Teacher::get_from_shortname(self.db.clone(), lb.clone().teacher).await.expect("gg");
            if let Some(teacher) = pot_teacher {
                for lesson in teacher.lessons {
                    additional_lbs.push(FormattedLesson {
                        teacher: teacher.shortname.clone(),
                        is_lb: true,
                        start: lb.clone().start,
                        length: 1,
                        day: lb.clone().day,
                        subject: lesson.to_string(),
                        subject_short: lesson.to_string(),
                        room: if new_room.clone() == "" {
                            lb.clone().room
                        } else {
                            new_room.clone()
                        },
                        substitution: lb.clone().substitution,
                    });
                }
            }
        }

        all_lbs = additional_lbs;

        let holidays = self
            .get_period_holidays(
                parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 529".into()))?,
                parameter.options.end_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 530".into()))?,
            )
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 533".into()))?;

        #[allow(clippy::type_complexity)]
        let mut lbs_per_week: HashMap<String, HashMap<String, Vec<(String, String, Option<Substitution>)>>> =
            HashMap::new();

        for lb in all_lbs {
            if !lb.is_lb {
                continue;
            };
            lbs_per_week
                .entry(lb.day.to_string() + ";" + &lb.start.to_string())
                .and_modify(|lessons| {
                    debug!("4 {:#?}", lessons);
                    lessons
                        .entry(lb.clone().subject_short)
                        .and_modify(|subject| {
                            subject.push((lb.teacher.to_string(), lb.room.to_string(), lb.clone().substitution))
                        })
                        .or_insert(vec![(lb.teacher.to_string(), lb.room.to_string(), lb.clone().substitution)]);
                })
                .or_insert(HashMap::from([(
                    lb.clone().subject_short,
                    vec![(lb.teacher.to_string(), lb.room.to_string(), lb.clone().substitution)],
                )]));
        }

        let mut every_lb: Vec<FormattedLesson> = vec![];

        for week_lesson in lbs_per_week {
            let lessons = week_lesson.1;
            let time: Vec<&str> = week_lesson.0.split(';').collect();
            let day = time[0].parse::<u8>().map_err(|err| Error::UntisError(err.to_string() + " 565".into()))?;
            let start = time[1].parse::<u8>().map_err(|err| Error::UntisError(err.to_string() + " 566".into()))?;

            for lesson in lessons {
                let mut teachers = "".to_string();
                let mut sub = "".to_string();
                let mut rooms = "".to_string();
                let mut cancelled = false;
                for subject in lesson.1 {
                    if teachers.contains(&subject.clone().0)
                        || (lesson.0 == "IF" && (*"O 2-16NT" != subject.1.clone() && *"H NT" != subject.1.clone()))
                    {
                        cancelled = true;
                        continue;
                    }
                    if !sub.is_empty() {
                        teachers += ", ";
                        rooms += ", ";
                    } else {
                        sub = subject.0.clone();
                    }
                    let mut new_room = subject.1;
                    if let Some(sub) = subject.2 {
                        if sub.cancelled {
                            cancelled = true;
                        }
                        if let Some(t) = sub.teacher {
                            if t == *"---" {
                                cancelled = true;
                            }
                        }
                        if let Some(room) = sub.room {
                            new_room = room;
                        }
                    }
                    if cancelled {
                        continue;
                    } else {
                        teachers += &subject.0;
                        rooms += &new_room;
                    }
                }
                if cancelled {
                    continue;
                }
                if rooms == *"" {
                    continue;
                } else {
                    every_lb.push(FormattedLesson {
                        teacher: teachers,
                        is_lb: true,
                        start,
                        length: 1,
                        day,
                        subject: lesson.0.clone(),
                        subject_short: lesson.0.clone(),
                        room: rooms,
                        substitution: None,
                    });
                }
            }
        }

        let mut formatted_holidays = self
            .format_lessons(holidays, parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 629".into()))?)
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 631".into()))?;

        every_lb.append(&mut formatted_holidays);

        Ok(every_lb)
    }

    pub async fn get_subjects(&mut self) -> Result<Vec<DetailedSubject>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getSubjects".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 640".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 642".into()))?;
        let json: UntisArrayResponse<DetailedSubject> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 643".into()))?;

        Ok(json.result)
    }

    pub async fn get_klassen(&mut self) -> Result<Vec<Klasse>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getKlassen".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 650".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 652".into()))?;
        let json: UntisArrayResponse<Klasse> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 653".into()))?;

        Ok(json.result)
    }

    pub async fn get_schoolyears(&mut self) -> Result<Vec<Schoolyear>, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getSchoolyears".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 662".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 664".into()))?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 665".into()))?;

        Ok(json.result)
    }

    pub async fn get_current_schoolyear(&mut self) -> Result<Schoolyear, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getCurrentSchoolyear".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 674".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 676".into()))?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 677".into()))?;
        let first = json.result[0].clone();

        Ok(first)
    }

    pub async fn get_holidays(&self) -> Result<Vec<Holidays>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getHolidays".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 685".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 687".into()))?;
        let json: UntisArrayResponse<Holidays> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 688".into()))?;

        Ok(json.result)
    }

    pub async fn get_timegrid_units(&self) -> Result<Vec<TimegridUnits>, Box<dyn std::error::Error>> {
        let response = self
            .request(utils::Parameter::Null(), "getTimegridUnits".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 697".into()))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 699".into()))?;
        let json: UntisArrayResponse<TimegridUnits> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 700".into()))?;

        Ok(json.result)
    }
}

impl Drop for UntisClient {
    fn drop(&mut self) {
        self.logout().expect("Error with the logout :)");
    }
}
