use std::collections::HashMap;

use reqwest::{Client, Response};
use reqwest::Error;

use crate::api_wrapper::utils::UntisResponse;

use super::utils::{self, LoginResults, TimetableParameter, UntisArrayResponse, PeriodObject, DetailedSubject, Schoolyear, Holidays, FormattedLesson, day_of_week, TimegridUnits, Substitution};

#[derive(Clone)]
pub struct UntisClient {
    pub person_type: u16,
    pub person_id: u16,
    id: String,
    school: String,
    subdomain: String,
    client: Client,
    jsessionid: String
}

#[allow(dead_code)]
impl UntisClient {
    async fn request(&mut self, params: utils::Parameter, method: String) -> Result<Response, Box<dyn std::error::Error>> {
        let body = utils::UntisBody {
            school: self.school.clone(),
            id: self.id.clone(),
            method,
            params,
            jsonrpc: "2.0".to_string()
        };

        let response = self.client.post(format!("https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}", self.subdomain, self.school))
            .body(serde_json::to_string(&body)?)
            .header("Cookie", "JSESSIONID=".to_owned() + &self.jsessionid)
            .send()
            .await?;
        
        Ok(response)
    }
    
    pub async fn init(user: String, password: String, id: String, school: String, subdomain: String) -> Result<Self, Box<dyn std::error::Error>> {
        
        let mut untis_client = Self {
            person_type: 0,
            person_id: 0,
            id,
            school,
            subdomain,
            client: Client::new(),
            jsessionid: "".to_string()
        };

        untis_client.login(user, password).await?;

        Ok(untis_client)
    }

    pub async fn unsafe_init(jsessionid: String, person_id: u16, person_type: u16,id: String, school: String, subdomain: String) -> Result<Self, Error> {
        let client = Client::new();
        
        let untis_client = Self {
            person_type,
            person_id,
            id,
            school,
            subdomain,
            client,
            jsessionid
        };

        Ok(untis_client)
    }

    async fn login(&mut self, user: String, password: String) -> Result<bool, Box<dyn std::error::Error>> {
        let params = utils::Parameter::AuthParameter(utils::AuthParameter {
            user,
            password,
            client: self.id.clone()
        });

        let response = self.request(
            params,
            "authenticate".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisResponse<LoginResults> = serde_json::from_str(&text)?;

        self.jsessionid = json.result.session_id;
        self.person_id = json.result.person_id;
        self.person_type = json.result.person_type;

        Ok(true)
    }

    pub fn logout(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let _reponse = self.request(
            utils::Parameter::Null(),
            "logout".to_string()
        );

        Ok(true)
    }

    pub async fn get_timetable(&mut self, parameter: TimetableParameter) -> Result<Vec<FormattedLesson>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::TimetableParameter(parameter), 
            "getTimetable".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<PeriodObject> = serde_json::from_str(&text)?;

        self.format_lessons(json.result).await
    }

    async fn format_lessons(&mut self, mut lessons: Vec<PeriodObject>) -> Result<Vec<FormattedLesson>, Box<dyn std::error::Error>> {
        let mut formatted: Vec<FormattedLesson> = vec![];

        lessons.sort_unstable_by_key(|les| les.date);
        let mut days: Vec<Vec<PeriodObject>> = vec![]; 

        let mut date = lessons[0].date;
        let mut day: Vec<PeriodObject> = vec![];

        for l in lessons {
            if l.date != date {
                day.sort_unstable_by_key(|ele| ele.start_time);
                let d = &day;
                days.push(d.to_owned());
                let new_date = &l.date;
                date = new_date.to_owned();
                day = vec![l];
            }
            else {
                day.push(l)
            }
        }
        day.sort_unstable_by_key(|ele| ele.start_time);
        let d = &day;
        days.push(d.to_owned());

        let mut skip: HashMap<u16, u8> = HashMap::new();
        let timegrid = self.get_timegrid_units().await?;
    
        for d in days {
            let clone = d.clone();
            for lesson in clone {
                if !lesson.su.is_empty() && skip.contains_key(&lesson.su[0].id) && skip[&lesson.su[0].id] > 0 {
                    skip.entry(lesson.su[0].id).and_modify(|skips| *skips -= 1);
                    if skip[&lesson.su[0].id] == 0 {
                        skip.remove(&lesson.su[0].id);
                    }
                    continue;
                }
                let day = day_of_week(lesson.date);
                let start = timegrid[usize::try_from(day)?].time_units.iter().position(|unit| unit.start_time == lesson.start_time).unwrap() + 1;
                let mut subject = "".to_string();
                let mut subject_short = "".to_string();
                match lesson.code.clone() {
                    Some(code) => {
                        
                        if code == "irregular" {
                            subject = match lesson.subst_text.clone(){
                                Some(text) => text,
                                None => "Entfällt".to_string()
                            }
                        }
                        else if code == "cancelled"{
                            continue;
                        }
                    },
                    None => {
                        if lesson.su.is_empty() {
                            if lesson.activity_type.is_none(){
                                match lesson.lstext {
                                    Some(text) => {
                                        subject = text.clone();
                                        let mut split: Vec<&str> = text.split_whitespace().collect();
                                        split.remove(0);
                                        split.remove(0);
                                        subject_short = split.join(" ");
                                    }
                                    None => {
                                        subject = "N/A".to_string();
                                        subject_short = "N/A".to_string();
                                    }
                                }
                            }
                            else{
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
                        else{
                            subject = lesson.su[0].name.to_owned();
                            subject_short = lesson.su[0].name.to_owned();
                            subject_short = subject_short.split(' ').collect::<Vec<&str>>()[0].to_owned();
                        }
                    }
                }
                let mut substituted = false;

                let teacher = if !lesson.te.is_empty() {
                    match lesson.te[0].orgname.clone() {
                        Some(orgname) => {
                            substituted = true;
                            orgname
                        },
                        None => {
                            lesson.te[0].name.to_owned()
                        }
                    }
                } else{ "".to_string() };
                
                let room = if !lesson.ro.is_empty() {
                    match lesson.ro[0].orgname.clone() {
                        Some(orgname) => {
                            substituted = true;
                            orgname
                        },
                        None => {
                            lesson.ro[0].name.to_owned()
                        }
                    }
                } else { "".to_string() };

                if lesson.code == Some("cancelled".to_string()) || lesson.subst_text.is_some() { substituted = true; }
                
                let mut formatted_lesson = FormattedLesson {
                    teacher,
                    is_lb: false,
                    start: u8::try_from(start)?,
                    length: if !lesson.su.is_empty() && d.iter().any(|les| !les.su.is_empty() && les.su[0].id == lesson.su[0].id && (les.start_time == lesson.end_time || les.start_time == lesson.end_time + 5)) {
                        if d.iter().any(|les| !les.su.is_empty() && les.su[0].id == lesson.su[0].id && (les.end_time == lesson.start_time || les.end_time == lesson.start_time - 5)) {
                            3
                        }
                        else{
                            2
                        }
                    }else if (lesson.end_time - lesson.start_time) > 85{
                        (((lesson.end_time - lesson.start_time) / 85) as f32).floor() as u8
                    }else{
                        1
                    },
                    day,
                    subject,
                    subject_short,
                    room,
                    substitution: if substituted { Some(Substitution {
                        teacher: if !lesson.te.is_empty() && lesson.te[0].orgname.is_some() {
                            if lesson.te[0].name.clone() == "---" {
                                None
                            } else {
                                Some(lesson.te[0].name.clone())
                            }
                        } else { None },
                        room: if !lesson.ro.is_empty() && lesson.ro[0].orgname.is_some() {
                            Some(lesson.ro[0].name.clone())
                        } else { None },
                        substition_text: lesson.subst_text.clone(),
                        cancelled: match lesson.code{
                            Some(code) => {
                                code == *"cancelled" || match lesson.subst_text{
                                    Some(text) => {
                                        text == *"Vtr. ohne Lehrer"
                                    }
                                    None => {
                                        false
                                    }
                                }
                            },
                            None => {
                                false
                            }
                        }
                    })} else { None }
                };
                formatted_lesson.is_lb = formatted_lesson.length == 1;
                if formatted_lesson.length > 1 && !lesson.su.is_empty() {
                    skip.insert(lesson.su[0].id, formatted_lesson.length - 1);
                }
                formatted.push(formatted_lesson);
            }
        }

        Ok(formatted)
    }

    pub async fn get_subjects(&mut self) -> Result<Vec<DetailedSubject>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::Null(),
            "getSubjects".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<DetailedSubject> = serde_json::from_str(&text)?;

        Ok(json.result)
    }

    pub async fn get_schoolyears(&mut self) -> Result<Vec<Schoolyear>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::Null(),
            "getSchoolyears".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text)?;

        Ok(json.result)
    }

    pub async fn get_current_schoolyear(&mut self) -> Result<Schoolyear, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::Null(),
            "getCurrentSchoolyear".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<Schoolyear> = serde_json::from_str(&text)?;
        let first = json.result[0].clone();

        Ok(first)
    }

    pub async fn get_holidays(&mut self) -> Result<Vec<Holidays>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::Null(),
            "getHolidays".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<Holidays> = serde_json::from_str(&text)?;

        Ok(json.result)
    }

    pub async fn get_timegrid_units(&mut self) -> Result<Vec<TimegridUnits>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::Null(),
            "getTimegridUnits".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<TimegridUnits> = serde_json::from_str(&text)?;

        Ok(json.result)
    }

}

impl Drop for UntisClient {
    fn drop(&mut self) {
        self.logout().expect("Error with the logout :)");
    }
}