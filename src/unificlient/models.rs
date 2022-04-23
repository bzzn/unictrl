use std::fmt;
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
    pub fn empty() -> UnifiStatusResult {
        UnifiStatusResult {
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
    pub fn empty() -> UnifiSitesResult {
        UnifiSitesResult {
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
    pub fn empty() -> UnifiActiveClientsResult {
        let mut empty = Vec::new();
        let data = UnifiActiveClientsData {
            mac: String::from(""),
            hostname: Some(String::from("")),
            note: Some(String::from("")),
            name: Some(String::from("")),
            network: String::from(""),
            ip: String::from(""),
            uptime: i32::from(0)
        };
        empty.push(data);
        return UnifiActiveClientsResult {
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
    pub network: String,
    pub ip: String,
    pub uptime: i32
}
