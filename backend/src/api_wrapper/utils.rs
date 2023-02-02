use serde::{Serialize, Deserialize};

use super::untis_client::UntisClient;



pub fn default_timetable(client: &UntisClient, start_date: String, end_date: String) -> TimetableParameter {
    TimetableParameter { 
        options: TimetableOptions {
            element: TimetableElement {
                id: client.person_id,
                r#type: client.person_type,
                key_type: "id".to_string()
            },
            start_date: start_date,
            end_date: end_date,
            only_base_timetable: false,
            show_booking: false,
            show_info: true,
            show_subst_text: true,
            show_ls_text: true,
            show_ls_number: true,
            show_studentgroup: false,
            klasse_fields: vec!["id".to_string(),"name".to_string(),"longname".to_string()],
            room_fields: vec!["id".to_string(),"name".to_string(),"longname".to_string()],
            subject_fields: vec!["id".to_string(),"name".to_string(),"longname".to_string()],
            teacher_fields: vec!["id".to_string(),"name".to_string(),"longname".to_string()], 
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Parameter{
    AuthParameter(AuthParameter),
    DateParameter(DateParameter),
    TimetableParameter(TimetableParameter),
    Null()
}



pub trait Result {}
pub trait ArrayResult {}



#[derive(Serialize, Deserialize, Debug)]
pub struct UntisBody {
    pub school: String,
    pub id: String,
    pub method: String,
    pub params: Parameter,
    pub jsonrpc: String
}

///
/// Authentification
/// 

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthParameter {
    pub user: String,
    pub password: String,
    pub client: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResults {
    pub session_id: String,
    pub person_type: u16,
    pub person_id: u16
}

impl Result for LoginResults {}

///
/// Date
///

#[derive(Serialize, Deserialize, Debug)]
pub struct DateParameter {
    pub options: DateOptions
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateOptions {
    pub start_date: String,
    pub end_date: String
}

///
/// Timetable
/// 

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimetableParameter {
    pub options: TimetableOptions
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimetableOptions {
    pub element: TimetableElement,
    pub start_date: String,
    pub end_date: String,
    pub only_base_timetable: bool,
    pub show_booking: bool,
    pub show_info: bool,
    pub show_subst_text: bool,
    pub show_ls_text: bool,
    pub show_ls_number: bool,
    pub show_studentgroup: bool,
    pub klasse_fields: Vec<String>,
    pub room_fields: Vec<String>,
    pub subject_fields: Vec<String>,
    pub teacher_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimetableElement {
    pub id: u16,
    pub r#type: u16,
    pub key_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Klasse {
    pub id: u16,
    pub name: String,
    pub longname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Teacher {
    pub id: u16,
    pub name: String,
    pub longname: String,
    #[serde(default)]
    pub orgid: Option<u16>,
    #[serde(default)]
    pub orgname: Option<String>
}
 
#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub id: u16,
    pub name: String,
    pub longname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub id: u16,
    pub name: String,
    pub longname: String,
    #[serde(default)]
    pub orgid: Option<u16>,
    #[serde(default)]
    pub orgname: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeriodObject {
    pub id: u32,
    pub date: u32,
    pub start_time: u16,
    pub end_time: u16,
    pub kl: Vec<Klasse>,
    pub te: Vec<Teacher>,
    pub su: Vec<Subject>,
    pub ro: Vec<Room>,
    pub activity_type: String,
    #[serde(default)]
    pub subst_text: Option<String>
}

impl ArrayResult for PeriodObject {}

///
/// Subjects
/// 

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetailedSubject {
    pub id: u16,
    pub name: String,
    pub longname: String,
    pub fore_color: String,
    pub back_color: String
}

impl ArrayResult for DetailedSubject {}

/// 
/// Schoolyear
/// 

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schoolyear{
    pub id: u16,
    pub name: String,
    pub start_date: String,
    pub end_date: String
}

impl ArrayResult for Schoolyear {}
impl Clone for Schoolyear {
    fn clone(&self) -> Self {  
        Schoolyear {
            id: self.id.clone(),
            name: self.name.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone()
        }
    }
}

/// 
/// Holiday
/// 

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Holidays {
    id: u16,
    name: String,
    longname: String,
    start_date: u16,
    end_date: u16
}

impl ArrayResult for Holidays {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UntisResponse<T> where T: Result {
    pub id: String,
    pub result: T,
    pub jsonrpc: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UntisArrayResponse<T> where T: ArrayResult {
    pub id: String,
    pub result: Vec<T>,
    pub jsonrpc: String
}