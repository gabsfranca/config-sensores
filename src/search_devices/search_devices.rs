//importando meus modulos
use crate::search_devices::discovery::tcp;
use crate::ip::ip_service;

use tokio::time::Duration;
use tokio::task;

//serve pra eu poder usar a mesma variavel em dois moves diferentes
use std::sync::Arc; //ARC = Atomic Reference Counted


#[tauri::command]
pub async fn search_devices() {
    let base_ip = initialize_base_ip().await;
    let base_ip = Arc::new(base_ip);

    let port = 2112;
    let timeout_duration = Duration::from_secs(2);

    let discover_task = task::spawn(discover_devices(Arc::clone(&base_ip), port, timeout_duration));

    if let Err(e) = discover_task.await {
        eprintln!("erro descoberta: {:?}", e);
    }
}


async fn initialize_base_ip() -> Option<String>{
    match ip::ip_service::get_local_ip() {
        Ok(ip) => {
            println!("ip: {}", &ip);
            let base_ip = ip::ip_service::remove_last_octet(&ip);
            Some(base_ip)
        }
        Err(e) => {
            eprintln!("erro ip inicial: {}", e);
            None
        }
    }
}

async fn discover_devices(base_ip: Arc<Option<String>>, port: u16, timeout: Duration) {
    if let Some(ip) = base_ip.as_ref() {
        let devices = crate::search_devices::discovery::tcp::discover_available_tcp_devices(ip, port, timeout).await;
        
        println!("dispositivos encontrados: ");
        println!("numero total: {}", devices.len());
        for device in devices {
            println!("{}", device);
        }
        
    } else {
        eprintln!("n achei nd")
    }
}