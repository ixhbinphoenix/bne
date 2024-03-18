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
            .await;

        debug!("{:?}", response);

        response.map_err(Error::Reqwest)
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

        untis_client.login(user, password).await.map_err(|err| Error::UntisError(err.to_string() + " 69"))?;
        untis_client.ids = untis_client.get_ids().await.map_err(|err| Error::UntisError(err.to_string() + " 70"))?;

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
        let klassen: Vec<Klasse> = self.get_klassen().await.map_err(|err| Error::UntisError(err.to_string() + " 118"))?;

        let ef_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "EF")
            .ok_or("Couldn't find EF")
            .map_err(|err| Error::UntisError(err.to_string() + " 125"))?;
        let q1_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "Q1")
            .ok_or("Couldn't find Q1")
            .map_err(|err| Error::UntisError(err.to_string() + " 131"))?;
        let q2_id = klassen
            .into_iter()
            .find(|klasse| klasse.name == "Q2")
            .ok_or("Couldn't find Q2")
            .map_err(|err| Error::UntisError(err.to_string() + " 136"))?;

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
            .map_err(|err| Error::UntisError(err.to_string() + " 155"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 157"))?;
        let json: UntisArrayResponse<PeriodObject> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 158"))?;

        let mut timetable = json.result;

        let mut holidays = self
            .get_period_holidays(
                parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 164"))?,
                parameter.options.end_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 165"))?,
            )
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 168"))?;

        timetable.append(&mut holidays);

        self.format_lessons(timetable, parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 172"))?)
            .await
    }

    pub async fn get_period_holidays(&self, start_date: u32, end_date: u32) -> Result<Vec<PeriodObject>, Error> {
        let all_holidays = self.get_holidays().await.map_err(|err| Error::UntisError(err.to_string() + " 172"))?;

        let holidays = all_holidays.iter().filter(|&holiday| {
            holiday.start_date <= i64::from(start_date) && holiday.end_date >= i64::from(start_date)
                || holiday.start_date <= i64::from(end_date) && holiday.end_date >= i64::from(start_date)
        });

        let mut period_holidays: Vec<PeriodObject> = vec![];

        for holiday in holidays {
            let start = NaiveDate::parse_from_str(&start_date.to_string(), "%Y%m%d").map_err(|err| Error::UntisError(err.to_string() + " 187"))?;
            let end = NaiveDate::parse_from_str(&end_date.to_string(), "%Y%m%d").map_err(|err| Error::UntisError(err.to_string() + " 188"))?;

            let length = end - start;

            for i in 0..=length.num_days() {
                if let Some(date) = start.checked_add_days(Days::new(i as u64)) {
                    if NaiveDate::parse_from_str(&holiday.start_date.to_string(), "%Y%m%d")
                        .map_err(|err| Error::UntisError(err.to_string() + " 195"))?
                        > date
                    {
                        continue;
                    }
                    if NaiveDate::parse_from_str(&holiday.end_date.to_string(), "%Y%m%d")
                        .map_err(|err| Error::UntisError(err.to_string() + " 201"))?
                        < date
                    {
                        break;
                    }
                    period_holidays.push(PeriodObject {
                        id: 1,
                        date: date.format("%Y%m%d").to_string().parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 208"))?,
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
        let timegrid = self.get_timegrid_units().await.map_err(|err| Error::UntisError(err.to_string() + " 258"))?;

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
                let start = timegrid[usize::from(day)]
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
                    start: u8::try_from(start).map_err(|err| Error::UntisError(err.to_string() + " 369"))?,
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

        // Get IDs of EF, Q1, Q2
        let ef_id = self.ids.get(&"EF".to_string()).ok_or("Couldn't find field EF").map_err(|err| Error::UntisError(err.to_string() + " 454"))?;
        let q1_id = self.ids.get(&"Q1".to_string()).ok_or("Couldn't find field Q1").map_err(|err| Error::UntisError(err.to_string() + " 455"))?;
        let q2_id = self.ids.get(&"Q2".to_string()).ok_or("Couldn't find field Q2").map_err(|err| Error::UntisError(err.to_string() + " 456"))?;

        parameter.options.element.r#type = 1;

        let mut ef_parameter = parameter.clone();
        let mut q1_parameter = parameter.clone();
        let mut q2_parameter = parameter.clone();

        ef_parameter.options.element.id = ef_id.to_owned();
        q1_parameter.options.element.id = q1_id.to_owned();
        q2_parameter.options.element.id = q2_id.to_owned();

        // Fetch timetables of EF, Q1, Q2 in parallel
        let ef_client = Arc::new(self.clone());
        future_lessons.spawn(async move { ef_client.clone().get_timetable(ef_parameter).await });
        let q1_client = Arc::new(self.clone());
        future_lessons.spawn(async move { q1_client.clone().get_timetable(q1_parameter).await });
        let q2_client = Arc::new(self.clone());
        future_lessons.spawn(async move { q2_client.clone().get_timetable(q2_parameter).await });

        let mut lessons: Vec<Vec<FormattedLesson>> = vec![];

        while let Some(res) = future_lessons.join_next().await {
            lessons.push(res.map_err(|err| Error::UntisError(err.to_string()))?.map_err(|err| Error::UntisError(err.to_string() + " 478"))?)
        }

        // Combine lernbueros of EF, Q1, Q2 into a single vec
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

        // Add additional subjects the Teacher does lernbueros for, as well as substitution info
        for lb in all_lbs.clone() {
            let mut new_room = "".to_string();
            // Checks for substitution on lesson, and if, sets the new room
            if let Some(sub) = &lb.substitution {
                if let Some(r) = &sub.room {
                    new_room = r.to_string();
                }
            }
            let pot_teacher = Teacher::get_from_shortname(self.db.clone(), lb.clone().teacher).await.expect("teacher shortname to exist in DB");
            if let Some(teacher) = pot_teacher {
                // Push a Version of the Lernbuero for every Subject to array
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
                parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 529"))?,
                parameter.options.end_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 530"))?,
            )
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 533"))?;

        #[allow(clippy::type_complexity)]
        // "My code is self-explained" MFs:
        // Holy shit there's a reason for that type complexity warning
        // HashMap (for weekdays+lesson) containing a HashMap (for subject) of
        // a Vec of (Teacher, Room, Option<SubstitutionInfo>)
        let mut lbs_per_week: HashMap<String, HashMap<String, Vec<(String, String, Option<Substitution>)>>> =
            HashMap::new();

        // Structures all LBs into a HashMap
        for lb in all_lbs {
            // Why tf is this check here? We would save so much computation if we checked this
            // beforehand
            // TODOO: Move this check to somewhere more sensible
            if !lb.is_lb {
                continue;
            };
            lbs_per_week
                .entry(lb.day.to_string() + ";" + &lb.start.to_string())
                .and_modify(|lessons| {
                    debug!("4 {:#?}", lessons);
                    lessons
                        .entry(lb.subject_short.to_string())
                        .and_modify(|subject| {
                            subject.push((lb.teacher.to_string(), lb.room.to_string(), lb.clone().substitution))
                        })
                        .or_insert(vec![(lb.teacher.to_string(), lb.room.to_string(), lb.clone().substitution)]);
                })
                .or_insert(HashMap::from([(
                    lb.clone().subject_short,
                    vec![(lb.teacher.to_string(), lb.room.to_string(), lb.substitution)],
                )]));
        }

        let mut every_lb: Vec<FormattedLesson> = vec![];

        // Merges LBs with the same Subject in parallel into a single FormattedLesson
        //  Lessons in a week
        for week_lesson in lbs_per_week {
            // See Type explanation in line 539
            let lessons = week_lesson.1; // HashMap of subject_short->LB
            let time: Vec<&str> = week_lesson.0.split(';').collect();
            let day = time[0].parse::<u8>().map_err(|err| Error::UntisError(err.to_string() + " 565"))?;
            let start = time[1].parse::<u8>().map_err(|err| Error::UntisError(err.to_string() + " 566"))?;

            // Irritatingly false variable name
            // subject_short->Vec<LB per Subject individually>
            // Loops over SUBJECTS???
            for lesson in lessons {

                let mut teachers = "".to_string();
                let mut sub = "".to_string();
                let mut rooms = "".to_string();

                let all_cancelled: bool = lesson.1.iter().all(|x| {
                    if let Some(sub) = &x.2 {
                        sub.cancelled ||
                        (sub.teacher.as_ref().is_some_and(|x| x == "---") && sub.substitution_text.as_ref().is_some_and(|x| x == "Vtr. ohne Lehrer"))
                    } else {
                        false
                    }
                });

                // Loops over LERNBUEROS of a SUBJECT????
                for subject in lesson.1 {
                    // If, for some reason, the teacher has multiple parallel lernbueros, we don't
                    // want to count them multiple times
                    if teachers.contains(&subject.clone().0) {
                        continue;
                    }
                    // If not every lesson is "cancelled" and this one is, skip this one
                    if !all_cancelled && subject.2.clone().is_some_and(|x| {
                        x.cancelled ||
                        (x.teacher.is_some_and(|y| y == "---") && x.substitution_text.is_some_and(|y| y == "Vtr. ohne Lehrer"))
                    }) {
                        continue;
                    }
                    // If there's no subject yet, set it
                    if sub.is_empty() {
                        sub = subject.0.clone();
                    } else { // If the subject is already set, we add another "," for seperation
                        teachers += ", ";
                        rooms += ", ";
                    }
                    let mut new_room = subject.1;
                    // Checks if there's a room change, and then instead adds that room
                    if let Some(sub) = subject.2 {
                        if let Some(room) = sub.room {
                            new_room = room;
                        }
                    }
                    // Adds the teacher and rooms to the already added ones.
                    teachers += &subject.0;
                    rooms += &new_room;
                }
                // Why does this check exist? This case is non-existent but go for it?
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
                        substitution: if all_cancelled {
                            Some(Substitution {
                                teacher: Some("---".to_string()),
                                room: None,
                                substitution_text: Some("Vtr. ohne Lehrer".to_string()),
                                cancelled: false
                            })
                        } else {
                            None
                        },
                    });
                }
            }
        }

        let mut formatted_holidays = self
            .format_lessons(holidays, parameter.options.start_date.parse::<u32>().map_err(|err| Error::UntisError(err.to_string() + " 629"))?)
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 631"))?;

        every_lb.append(&mut formatted_holidays);

        Ok(every_lb)
    }

    pub async fn get_subjects(&mut self) -> Result<Vec<DetailedSubject>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getSubjects".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 640"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 642"))?;
        let json: UntisArrayResponse<DetailedSubject> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 643"))?;

        Ok(json.result)
    }

    pub async fn get_klassen(&mut self) -> Result<Vec<Klasse>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getKlassen".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 650"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 652"))?;
        let json: UntisArrayResponse<Klasse> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 653"))?;

        Ok(json.result)
    }

    pub async fn get_schoolyears(&mut self) -> Result<Vec<Schoolyear>, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getSchoolyears".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 662"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 664"))?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 665"))?;

        Ok(json.result)
    }

    pub async fn get_current_schoolyear(&mut self) -> Result<Schoolyear, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getCurrentSchoolyear".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 674"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 676"))?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 677"))?;
        let first = json.result[0].clone();

        Ok(first)
    }

    pub async fn get_holidays(&self) -> Result<Vec<Holidays>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getHolidays".to_string()).await.map_err(|err| Error::UntisError(err.to_string() + " 685"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 687"))?;
        let json: UntisArrayResponse<Holidays> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 688"))?;

        Ok(json.result)
    }

    pub async fn get_timegrid_units(&self) -> Result<Vec<TimegridUnits>, Box<dyn std::error::Error>> {
        let response = self
            .request(utils::Parameter::Null(), "getTimegridUnits".to_string())
            .await
            .map_err(|err| Error::UntisError(err.to_string() + " 697"))?;

        let text = response.text().await.map_err(|err| Error::UntisError(err.to_string() + " 699"))?;
        let json: UntisArrayResponse<TimegridUnits> = serde_json::from_str(&text).map_err(|err| Error::UntisError(err.to_string() + " 700"))?;

        Ok(json.result)
    }
}

impl Drop for UntisClient {
    fn drop(&mut self) {
        self.logout().expect("Error with the logout :)");
    }
}
