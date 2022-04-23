use reqwest::blocking::Client;

pub mod models;
use { models::UnifiClientError, models::UnifiUser, models::UnifiStatusResult, models::UnifiSitesResult, models::UnifiActiveClientsResult };

pub struct UnifiClient {
    host: String,
    client: reqwest::blocking::Client,
}

impl UnifiClient {
    pub fn new(host: String) -> UnifiClient {
        return UnifiClient {
            host: host,
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .cookie_store(true)
                .build()
                .unwrap()
        }
    }

    fn address(&self, path: &str) -> String {
        return format!("{}{}", &self.host, path);
    }

    pub fn status(&self) -> UnifiStatusResult {
        let address = &format!("{}/status", &self.host);
        let result = self.client.get(address).send();
    
        return match result {
            Ok(v) => v.json::<UnifiStatusResult>().unwrap(),
            Err(_) => UnifiStatusResult::empty()
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<bool, UnifiClientError> {
        let address = &self.address("/api/login");
        let user = UnifiUser {
            username: String::from(username),
            password: String::from(password)
        };

        let result = &self.client
            .post(address)
            .json::<UnifiUser>(&user)
            .send();

        return match result {
            Ok(r) => {
                let auth = r.status() == 200;
                if auth {
                    Ok(auth)
                } else {
                    Err(UnifiClientError::AuthenticationFailure)
                }
            },
            Err(_) => Err(UnifiClientError::ControllerUnreachable)
        };
    }
    
    pub fn sites(&self) -> UnifiSitesResult {
        let address = &self.address("/api/self/sites");
        let result = self.client.get(address).send();

        return match result {
            Ok(r) => r.json::<UnifiSitesResult>().unwrap(),
            Err(_) => UnifiSitesResult::empty()
        };
    }

    pub fn active_clients(&self, site: &str) -> UnifiActiveClientsResult  {
        let address = &self.address(&format!("/api/s/{}/stat/sta", site));
        let result = self.client.get(address).send();

        return match result {
            Ok(r) => r.json::<UnifiActiveClientsResult>().unwrap(),
            Err(_) => UnifiActiveClientsResult::empty()
        }
    }
}
