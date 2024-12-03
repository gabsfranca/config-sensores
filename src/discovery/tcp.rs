//importando o tipo SocketAddr, que é uma struct que representa um endereço de rede(ip + porta)
use std::net::SocketAddr;

//tcpstream pra fazer coisa assincrona de rede TCP 
use tokio::net::TcpStream;

//duration serve pra especificar tempo no rust,
//meio que o timeout precisa ser importado junto com o duration(pelo q eu entendi)
use tokio::time::{timeout, Duration};

pub async fn discover_tcp_devices(base_ip: &str, port: u16, timeout_duration: Duration) -> Vec<String> {
    //criando um array de tasks assincronas
    let mut tasks = Vec::new();

    //pegando todos os ips possiveis da faixa
    for i in 1..254 {
        let ip = format!("{}.{}", base_ip, i); 
        //pegando a strring do ip e tranformando num socketaddr ou retornando um erro
        let addr: SocketAddr = format!("{}:{}", ip, port)
            .parse()
            .expect("nao consegui construir o endereço, mim desculpa");

        //pelo que eu entendi o move ele pega os valores e "empresta"
        //pra garantir que outras coisas nao user aquela variavel(no caso o ip )
        let task = tokio::spawn(async move {
            if timeout(timeout_duration, TcpStream::connect(addr)).await.is_ok() {
                Some(ip)
            } else {
                None
            }
        });

        tasks.push(task)
    
    }

    let mut devices = Vec::new();
    for task in tasks {
        if let Ok(Some(ip)) = task.await {
            devices.push(ip);
        }
    }

    devices

}

pub async fn discover_available_tcp_devices(base_ip: &str, port: u16, timeout_duration: Duration) -> Vec<String> {
    //criando um array de tasks assincronas
    let mut tasks = Vec::new();

    //pegando todos os ips possiveis da faixa
    for i in 1..254 {
        let ip = format!("{}.{}", base_ip, i);
        //pegando a strring do ip e tranformando num socketaddr ou retornando um erro
        let addr: SocketAddr = format!("{}:{}", ip, port)
            .parse()
            .expect("nao consegui construir o endereço, mim desculpa");

        //pelo que eu entendi o move ele pega os valores e "empresta"
        //pra garantir que outras coisas nao user aquela variavel(no caso o ip )
        let task = tokio::spawn(async move {
            if let Ok(Ok(stram)) = timeout(timeout_duration, TcpStream::connect(addr)).await {
                if stram.peer_addr().is_ok() {
                    Some(ip)
                } else {
                    None
                }
            }else {
                None
            }
        });

        tasks.push(task)
    }

    let mut devices = Vec::new();
    for task in tasks {
        if let Ok(Some(ip)) = task.await {
            devices.push(ip);
        }
    }

    devices

}