use std::{collections::HashMap, sync::Arc};

use actix_web_lab::__reexports::tokio::task::JoinSet;
use chrono::{Days, NaiveDate};
use reqwest::{Client, Response};

use super::utils::{
    self, day_of_week, DetailedSubject, FormattedLesson, Holidays, Klasse, LoginResults, PeriodObject, Schoolyear, Substitution, TimegridUnits, TimetableParameter, UntisArrayResponse
};
use crate::{api_wrapper::utils::UntisResponse, prelude::Error};

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
            .body(serde_json::to_string(&body).map_err(|_| Error::UntisError)?)
            .header("Cookie", "JSESSIONID=".to_owned() + &self.jsessionid)
            .send()
            .await
            .map_err(|err| Error::Reqwest(err))?;

        Ok(response)
    }

    pub async fn init(
        user: String, password: String, id: String, school: String, subdomain: String,
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
        };

        untis_client.login(user, password).await.map_err(|_| Error::UntisError)?;
        untis_client.ids = untis_client.get_ids().await.map_err(|_| Error::UntisError)?;

        Ok(untis_client)
    }

    pub async fn unsafe_init(
        jsessionid: String, person_id: u16, person_type: u16, id: String, school: String, subdomain: String,
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
        let klassen: Vec<Klasse> = self.get_klassen().await.map_err(|_| Error::UntisError)?;

        let ef_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "EF")
            .ok_or("Couldn't find EF")
            .map_err(|_| Error::UntisError)?;
        let q1_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "Q1")
            .ok_or("Couldn't find Q1")
            .map_err(|_| Error::UntisError)?;
        let q2_id = klassen
            .clone()
            .into_iter()
            .find(|klasse| klasse.name == "Q2")
            .ok_or("Couldn't find Q2")
            .map_err(|_| Error::UntisError)?;

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
            .map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<PeriodObject> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        let mut timetable = json.result;

        let mut holidays = self
            .get_period_holidays(
                parameter.options.start_date.parse::<u32>().map_err(|_| Error::UntisError)?,
                parameter.options.end_date.parse::<u32>().map_err(|_| Error::UntisError)?,
            )
            .await
            .map_err(|_| Error::UntisError)?;

        timetable.append(&mut holidays);

        self.format_lessons(timetable, parameter.options.start_date.parse::<u32>().map_err(|_| Error::UntisError)?)
            .await
    }

    pub async fn get_period_holidays(&self, start_date: u32, end_date: u32) -> Result<Vec<PeriodObject>, Error> {
        let all_holidays = self.get_holidays().await.map_err(|_| Error::UntisError)?;

        let holidays = all_holidays.iter().filter(|&holiday| {
            holiday.start_date <= i64::from(start_date) && holiday.end_date >= i64::from(start_date)
                || holiday.start_date <= i64::from(end_date) && holiday.end_date >= i64::from(start_date)
        });

        let mut period_holidays: Vec<PeriodObject> = vec![];

        for holiday in holidays {
            let start = NaiveDate::parse_from_str(&start_date.to_string(), "%Y%m%d").map_err(|_| Error::UntisError)?;
            let end = NaiveDate::parse_from_str(&end_date.to_string(), "%Y%m%d").map_err(|_| Error::UntisError)?;

            let length = end - start;

            for i in 0..=length.num_days() {
                if let Some(date) = start.checked_add_days(Days::new(i as u64)) {
                    if NaiveDate::parse_from_str(&holiday.start_date.to_string(), "%Y%m%d")
                        .map_err(|_| Error::UntisError)?
                        > date
                    {
                        continue;
                    }
                    if NaiveDate::parse_from_str(&holiday.end_date.to_string(), "%Y%m%d")
                        .map_err(|_| Error::UntisError)?
                        < date
                    {
                        break;
                    }
                    period_holidays.push(PeriodObject {
                        id: 0,
                        date: date.format("%Y%m%d").to_string().parse::<u32>().map_err(|_| Error::UntisError)?,
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
        let timegrid = self.get_timegrid_units().await.map_err(|_| Error::UntisError)?;

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
                let start = timegrid[usize::try_from(day).map_err(|_| Error::UntisError)?]
                    .time_units
                    .iter()
                    .position(|unit| unit.start_time == lesson.start_time)
                    .unwrap()
                    + 1;
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
                    start: u8::try_from(start).map_err(|_| Error::UntisError)?,
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
                        (((lesson.end_time - lesson.start_time) / 85) as f32).floor() as u8
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

    pub async fn get_lernbueros(&mut self, mut parameter: TimetableParameter) -> Result<Vec<FormattedLesson>, Error> {
        let mut all_lbs: Vec<FormattedLesson> = vec![];
        let mut future_lessons = JoinSet::new();

        let ef_id = self.ids.get(&"EF".to_string()).ok_or("Couldn't find field EF").map_err(|_| Error::UntisError)?;
        let q1_id = self.ids.get(&"Q1".to_string()).ok_or("Couldn't find field Q1").map_err(|_| Error::UntisError)?;
        let q2_id = self.ids.get(&"Q2".to_string()).ok_or("Couldn't find field Q2").map_err(|_| Error::UntisError)?;

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
            lessons.push(res.map_err(|_| Error::UntisError)?.map_err(|_| Error::UntisError)?)
        }

        all_lbs.append(
            &mut lessons[0].clone().into_iter().filter(|lesson| lesson.is_lb == true).collect::<Vec<FormattedLesson>>(),
        );
        all_lbs.append(
            &mut lessons[1].clone().into_iter().filter(|lesson| lesson.is_lb == true).collect::<Vec<FormattedLesson>>(),
        );
        all_lbs.append(
            &mut lessons[2].clone().into_iter().filter(|lesson| lesson.is_lb == true).collect::<Vec<FormattedLesson>>(),
        );

        let holidays = self
            .get_period_holidays(
                parameter.options.start_date.parse::<u32>().map_err(|_| Error::UntisError)?,
                parameter.options.end_date.parse::<u32>().map_err(|_| Error::UntisError)?,
            )
            .await
            .map_err(|_| Error::UntisError)?;

        let mut formatted_holidays = self
            .format_lessons(holidays, parameter.options.start_date.parse::<u32>().map_err(|_| Error::UntisError)?)
            .await
            .map_err(|_| Error::UntisError)?;

        all_lbs.append(&mut formatted_holidays);

        Ok(all_lbs)
    }

    pub async fn get_subjects(&mut self) -> Result<Vec<DetailedSubject>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getSubjects".to_string()).await.map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<DetailedSubject> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        Ok(json.result)
    }

    pub async fn get_klassen(&mut self) -> Result<Vec<Klasse>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getKlassen".to_string()).await.map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<Klasse> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        Ok(json.result)
    }

    pub async fn get_schoolyears(&mut self) -> Result<Vec<Schoolyear>, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getSchoolyears".to_string())
            .await
            .map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        Ok(json.result)
    }

    pub async fn get_current_schoolyear(&mut self) -> Result<Schoolyear, Error> {
        let response = self
            .request(utils::Parameter::Null(), "getCurrentSchoolyear".to_string())
            .await
            .map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;
        let first = json.result[0].clone();

        Ok(first)
    }

    pub async fn get_holidays(&self) -> Result<Vec<Holidays>, Error> {
        let response =
            self.request(utils::Parameter::Null(), "getHolidays".to_string()).await.map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<Holidays> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        Ok(json.result)
    }

    pub async fn get_timegrid_units(&self) -> Result<Vec<TimegridUnits>, Box<dyn std::error::Error>> {
        let response = self
            .request(utils::Parameter::Null(), "getTimegridUnits".to_string())
            .await
            .map_err(|_| Error::UntisError)?;

        let text = response.text().await.map_err(|_| Error::UntisError)?;
        let json: UntisArrayResponse<TimegridUnits> = serde_json::from_str(&text).map_err(|_| Error::UntisError)?;

        Ok(json.result)
    }
}

impl Drop for UntisClient {
    fn drop(&mut self) {
        self.logout().expect("Error with the logout :)");
    }
}
