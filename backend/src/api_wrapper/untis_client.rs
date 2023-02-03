use reqwest::{Client, Response};
use reqwest::Error;

use crate::api_wrapper::utils::UntisResponse;

use super::utils::{self, LoginResults, TimetableParameter, UntisArrayResponse, PeriodObject, DetailedSubject, Schoolyear, Holidays};

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
    
    pub async fn init(user: String, password: String, id: String, school: String, subdomain: String) -> Result<Self, Box<dyn std::error::Error>>{
        
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

    pub async fn get_timetable(&mut self, parameter: TimetableParameter) -> Result<Vec<PeriodObject>, Box<dyn std::error::Error>> {
        let response = self.request(
            utils::Parameter::TimetableParameter(parameter), 
            "getTimetable".to_string()
        ).await?;

        let text = response.text().await?;
        let json: UntisArrayResponse<PeriodObject> = serde_json::from_str(&text)?;

        Ok(json.result)
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

}

impl Drop for UntisClient {
    fn drop(&mut self) {
        self.logout().expect("Error with the logout :)");
    }
}
