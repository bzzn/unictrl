use std::env;

include!("models.rs");
include!("unificlient.rs");

fn main() {
    let host: &str = &env::var("UNICTRL_HOST").unwrap_or("".to_string());
    let username: &str = &env::var("UNICTRL_USERNAME").unwrap_or("".to_string());
    let password: &str = &env::var("UNICTRL_PASSWORD").unwrap_or("".to_string());

    if !host.starts_with("http") {
        println!("Controller host not set or invalid: {}", host);
        return;
    }

    let unifi_client = UnifiClient::new(String::from(host));
    let host_status = unifi_client.status();
    let host_state: &str = match host_status.meta.up {
        true => "Up",
           _ => "Down"
    };
    println!("Controller {} is {} - Version: {})", host, host_state, host_status.meta.server_version);

    let logged_in: bool = unifi_client.login(username, password);
    if !logged_in {
        println!("Login failed for user: {} (password set: {})", username, password.len() > 0);
        return;
    }

    let si = unifi_client.sites();
    println!("Site: {} ({}) [{}]", si.data[0].name, si.data[0].desc, si.data[0]._id);

    let active_clients = unifi_client.active_clients(si.data[0].name.as_str());
    for client in active_clients.data {
        let hostname = match client.hostname {
            Some(n) => n,
            None => String::from("n/a")
        };
        let name = match client.name {
            Some(n) => n,
            None => String::from("n/a")
        };
        let note = match client.note {
            Some(n) => n,
            None => String::from("n/a")
        };
        println!("{:17} {:<15} {:<25} {:<20} {:<20}", client.mac, client.ip, hostname, name, note);
    }
}
