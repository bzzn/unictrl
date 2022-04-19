use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UnifiMeta {
    rc: String,
    up: bool,
    server_version: String,
    uuid: String
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiStatusResult {
    meta: UnifiMeta,
}

impl UnifiStatusResult {
    fn empty() -> UnifiStatusResult {
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
struct UnifiUser {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiLoginResultMeta {
    rc: String,
    msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiLoginResult {
    meta: UnifiLoginResultMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiSitesMeta {
    rc: String,
    msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiSitesData {
    _id: String,
    name: String,
    desc: String,
    role: String
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiSitesResult {
    meta: UnifiSitesMeta,
    data: Vec<UnifiSitesData>
}

impl UnifiSitesResult {
    fn empty() -> UnifiSitesResult {
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
struct UnifiActiveClientsResult {
    data: Vec<UnifiActiveClientsData>
}

impl UnifiActiveClientsResult {
    fn empty() -> UnifiActiveClientsResult {
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
struct UnifiActiveClientsData {
    mac: String,
    hostname: Option<String>,
    note: Option<String>,
    name: Option<String>,
    network: String,
    ip: String,
    uptime: i32
}
