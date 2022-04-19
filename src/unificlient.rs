use reqwest::blocking::Client;

struct UnifiClient {
    host: String,
    client: reqwest::blocking::Client,
}

impl UnifiClient {
    fn new(host: String) -> UnifiClient {
        return UnifiClient {
            host: host,
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .cookie_store(true)
                .build()
                .unwrap()
        }
    }

    fn status(&self) -> UnifiStatusResult {
        let address = &format!("{}/status", &self.host);
        let result = self.client.get(address).send();
    
        return match result {
            Ok(v) => v.json::<UnifiStatusResult>().unwrap(),
            Err(_) => UnifiStatusResult::empty()
        }
    }

    fn login(&self, username: &str, password: &str) -> bool {
        let path: &str = "/api/login";
        let address = format!("{}{}", &self.host, path);
        let user: UnifiUser = UnifiUser {
            username: String::from(username),
            password: String::from(password)
        };
        let result = &self.client
            .post(address)
            .json::<UnifiUser>(&user)
            .send();

        return match result {
            Ok(r) => r.status() == 200,
            Err(e) => panic!("Login failed: {}", e)
        }
    }
    
    fn sites(&self) -> UnifiSitesResult {
        let path: &str = "/api/self/sites";
        let address = format!("{}{}", &self.host, path);
        let result = self.client.get(address).send();

        return match result {
            Ok(r) => r.json::<UnifiSitesResult>().unwrap(),
            Err(_) => UnifiSitesResult::empty()
        };
    }

    fn active_clients(&self, site: &str) -> UnifiActiveClientsResult  {
        let path: &str = &format!("/api/s/{}/stat/sta", site);
        let address = format!("{}{}", &self.host, path);
        let result = self.client.get(address).send();

        return match result {
            Ok(r) => r.json::<UnifiActiveClientsResult>().unwrap(),
            Err(_) => UnifiActiveClientsResult::empty()
        }
    }
}
