use std::fmt;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub enum UnifiClientError {
    ControllerUnreachable,
    AuthenticationFailure
}

impl fmt::Display for UnifiClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnifiClientError::ControllerUnreachable => write!(f, "Controller unreachable"),
            UnifiClientError::AuthenticationFailure => write!(f, "Authentication failed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiMeta {
    pub rc: String,
    pub up: bool,
    pub server_version: String,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiStatusResult {
    pub meta: UnifiMeta,
}

impl UnifiStatusResult {
    pub fn empty() -> Self {
        Self {
            meta: UnifiMeta {
                rc: String::from(""),
                up: false,
                server_version: String::from(""),
                uuid: String::from("")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiUser {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiLoginResultMeta {
    pub rc: String,
    pub msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiLoginResult {
    pub meta: UnifiLoginResultMeta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiSitesMeta {
    pub rc: String,
    pub msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiSitesData {
    pub _id: String,
    pub name: String,
    pub desc: String,
    pub role: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiSitesResult {
    pub meta: UnifiSitesMeta,
    pub data: Vec<UnifiSitesData>
}

impl UnifiSitesResult {
    pub fn empty() -> Self {
        Self {
            meta: UnifiSitesMeta {
                rc: String::from(""),
                msg: Some(String::from(""))
            },
            data: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiActiveClientsResult {
    pub data: Vec<UnifiActiveClientsData>
}

impl UnifiActiveClientsResult {
    pub fn empty() -> Self {
        let mut empty = Vec::new();
        let data = UnifiActiveClientsData {
            mac: String::from(""),
            hostname: Some(String::from("")),
            note: Some(String::from("")),
            name: Some(String::from("")),
            network: Some(String::from("")),
            ip: Some(String::from("")),
            uptime: i32::from(0)
        };
        empty.push(data);
        return Self {
            data: empty
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiActiveClientsData {
    pub mac: String,
    pub hostname: Option<String>,
    pub note: Option<String>,
    pub name: Option<String>,
    pub network: Option<String>,
    pub ip: Option<String>,
    pub uptime: i32
}

impl fmt::Display for UnifiActiveClientsData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let not_avail = String::from("n/a");
        return write!(f, "{:<16} {:<16} {:<16} {:<16}",
            &self.mac,
            &self.ip.as_ref().unwrap_or(&not_avail),
            &self.name.as_ref().unwrap_or(&not_avail),
            &self.note.as_ref().unwrap_or(&not_avail),
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiAllClientsResult {
    pub data: Vec<UnifiAllClientsData>
}

impl UnifiAllClientsResult {
    pub fn empty() -> Self {
        let mut empty = Vec::new();
        let data = UnifiAllClientsData {
            mac: String::from(""),
            hostname: Some(String::from("")),
            note: Some(String::from("")),
            name: Some(String::from("")),
            network: Some(String::from("")),
            ip: Some(String::from("")),
            // uptime: i32::from(0)
        };
        empty.push(data);
        return Self {
            data: empty
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiAllClientsData {
    pub mac: String,
    pub hostname: Option<String>,
    pub note: Option<String>,
    pub name: Option<String>,
    pub network: Option<String>,
    pub ip: Option<String>,
    // pub uptime: i32
}

impl fmt::Display for UnifiAllClientsData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let not_avail = String::from("n/a");
        return write!(f, "{:<16} {:<16} {:<16} {:<16}",
            &self.mac,
            &self.ip.as_ref().unwrap_or(&not_avail),
            &self.name.as_ref().unwrap_or(&not_avail),
            &self.note.as_ref().unwrap_or(&not_avail),
        );
    }
}

pub struct UnifiClient {
    host: String,
    client: reqwest::blocking::Client,
}

impl UnifiClient {
    pub fn new(host: String) -> Self {
        Self {
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
                match r.status() {
                    reqwest::StatusCode::OK => Ok(true),
                    _ => Err(UnifiClientError::AuthenticationFailure),
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

    pub fn all_clients(&self, site: &str) -> UnifiAllClientsResult  {
        let address = &self.address(&format!("/api/s/{}/stat/device", site));
        let result = self.client.get(address).send();

        return match result {
            Ok(r) => r.json::<UnifiAllClientsResult>().unwrap(),
            Err(_) => UnifiAllClientsResult::empty()
        }
    }
}
