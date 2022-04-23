use std::env;

mod unificlient;
use unificlient::UnifiClient;

fn main() {
    let host = &env::var("UNICTRL_HOST").unwrap_or("".to_string());
    let username = &env::var("UNICTRL_USERNAME").unwrap_or("".to_string());
    let password = &env::var("UNICTRL_PASSWORD").unwrap_or("".to_string());

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
    let mut abort = false;
    match unifi_client.login(username, password) {
        Ok(r) => authenticated = r,
        Err(err) => { abort = true; println!("ER: {}", err); },
    };

    if abort {
        println!("Exiting...");
        return;
    }

    if !authenticated {
        println!("Login failed for user: {} (password set: {})", username, password.len() > 0);
        return;
    }

    let si = unifi_client.sites();
    println!("Site: {} ({}) [{}]", si.data[0].name, si.data[0].desc, si.data[0]._id);

    let active_clients = unifi_client.active_clients(si.data[0].name.as_str());
    for client in active_clients.data {
        let hostname = client.hostname.unwrap_or("n/a".to_string());
        let name = client.name.unwrap_or("n/a".to_string());
        let note = client.note.unwrap_or("n/a".to_string());
        println!("{:17} {:<15} {:<25} {:<20} {:<20}", client.mac, client.ip, hostname, name, note);
    }
}
