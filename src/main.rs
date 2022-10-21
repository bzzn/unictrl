extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::fmt;

mod unificlient;
use { unificlient::UnifiClient, unificlient::UnifiClientError };

fn make_local_name(host: &str, domain: &str) -> String {
    let mut name = host.to_lowercase();
    name = name.replace(" ", "-");
    let local_domain = domain;
    return format!("{}.{}", name, local_domain);
}

struct HostRecord {
    dns_name: String,
    ip_addr: String
}

impl fmt::Display for HostRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:<16} {}",
            &self.ip_addr,
            &self.dns_name
        );
    }
}

fn main() {
    dotenv().expect("Failed loading env vars");
    let host = &env::var("UNICTRL_HOST").unwrap_or("".to_string());
    let username = &env::var("UNICTRL_USERNAME").unwrap_or("".to_string());
    let password = &env::var("UNICTRL_PASSWORD").unwrap_or("".to_string());
    let domain = &env::var("UNICTRL_DOMAIN").unwrap_or("".to_string());

    if !host.starts_with("http") {
        println!("Controller host not set or invalid: {}", host);
        return;
    }

    let unifi_client = UnifiClient::new(String::from(host));
    let host_status = unifi_client.status();
    let host_state = match host_status.meta.up {
        true => "Up",
           _ => "Down"
    };
    println!("Controller {} is {} - Version: {})", host, host_state, host_status.meta.server_version);

    let mut authenticated = false;
    match unifi_client.login(username, password) {
        Ok(r) => authenticated = r,
        Err(err) => match err {
            UnifiClientError::AuthenticationFailure => { println!("Login: {}", err); },
            UnifiClientError::ControllerUnreachable => { println!("Login: {}", err); },
        },
    };

    if !authenticated {
        println!("Login failed for user: {} (password set: {})", username, password.len() > 0);
        return;
    }

    let si = unifi_client.sites();
    println!("Site: {} ({}) [{}]", si.data[0].name, si.data[0].desc, si.data[0]._id);

    let active_clients = unifi_client.active_clients(si.data[0].name.as_str());
    let filtered_clients: Vec<HostRecord> = active_clients.data
        .iter()
        .filter(|h| h.name != None && h.name != Some(String::from("")))
        .map(|client| HostRecord {
            dns_name: make_local_name(client.name.clone().unwrap().as_str(), domain),
            ip_addr: match &client.ip {
                Some(ip) => ip.clone(),
                _ => String::from("# no ip")
            }
        })
        .collect();

    println!("# ACTIVE CLIENTS");
    for client in filtered_clients {
        println!("{}", client);
    }

    let all_clients = unifi_client.all_clients(si.data[0].name.as_str());

    println!("# ALL CLIENTS");
    for client in all_clients.data.iter() {
        println!("{}", client);
    }

    let all_filtered_clients: Vec<HostRecord> = all_clients.data
        .iter()
        .filter(|h| h.name != None && h.name != Some(String::from("")))
        .map(|client| HostRecord {
            dns_name: make_local_name(client.name.clone().unwrap().as_str(), domain),
            ip_addr: match &client.ip {
                Some(ip) => ip.clone(),
                _ => String::from("# no ip")
            }
        })
        .collect();

    println!("# ALL CLIENTS -- FILTERED");
    for client in all_filtered_clients {
        println!("{}", client);
    }
}
