use reqwest::{Client, Response};
use reqwest::Error;

use super::utils::{self};

#[derive(Clone)]
pub struct UntisClient {
    user: String,
    password: String,
    id: String,
    school: String,
    subdomain: String,
    client: Client,
    jsessionid: String
}

impl UntisClient {

    async fn request(&mut self, params: utils::Parameter, method: String) -> Result<Response, Error> {
        let body = utils::UntisBody {
            school: self.school.clone(),
            id: self.id.clone(),
            method: method,
            params: params,
            jsonrpc: "2.0".to_string()
        };
        let response = self.client.post(format!("https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}", self.subdomain, self.school))
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await?;
        Ok(response)
    }

    pub async fn init(user: String, password: String, id: String, school: String, subdomain: String) -> Result<Self, Error>{
        let user = user.clone();
        let password = password.clone();
        let id = id.clone();
        let school = school.clone();
        let client = Client::new();
        let subdomain = subdomain.clone();
        let jsessionid = "".to_string();

        let mut untis_client = Self { user: user, password: password, id: id, school: school, subdomain: subdomain, client: client, jsessionid: jsessionid };

        untis_client.login().await;

        Ok(untis_client)
    }

    async fn login(&mut self) {

        let params = utils::Parameter::AuthParameter(utils::AuthParameter {
            user: self.user.clone(),
            password: self.password.clone(),
            client: self.id.clone()
        });

        let response = self.request(params, "authenticate".to_string()).await.expect("Request failed");
        let text = response.text().await.expect("No Response");

        println!("{:#?}", text);
    }
}
