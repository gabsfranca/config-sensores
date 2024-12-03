use local_ip_address::local_ip;

pub fn get_local_ip() -> Result<String, String> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(e) => Err(format!("erro ao obter endereço ip do local: {}", e)),
    }
}

pub fn remove_last_octet(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        let ip_without_last = parts[0..3].join(".");
        ip_without_last
    } else {
        String::new()
    }
}