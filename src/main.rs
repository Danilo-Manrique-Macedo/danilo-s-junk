use reqwest;
use std::{error::Error, thread, time::Duration};
use clearscreen;
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        clearscreen::clear().unwrap();
        println!("<----------Detector de IPv4 e IPv6--------->");
        println!("Versão:1.0");
        println!("Feito com RUST <3");
        println!("---------------API's usadas----------------");
        println!("API's usadas para detectar os IP's públicos:");
        println!("IPv4: api.ipify.org");
        println!("IPv6: api6.ipify.org");
        println!("----------Informações da detecção----------");

        // Detecção do IPv4
        let url4 = "https://api.ipify.org";
        let response4 = reqwest::blocking::get(url4)?;

        // Impressão do IPv4
        if response4.status().is_success() {
            let data4 = response4.text()?;
            println!("Seu endereço IPv4 público é: {}", data4);
        } else {
            println!("ERRO: Falha em tentar obter o endereço IPv4.");
            println!("Causas: Você pode estar sem um endereço IPv4, totalmente desconectado da internet ou suas configurações de rede podem estar mal configuradas.");
            println!("Possível Solução: Verifique suas configurações de rede e/ou cabos.");
        }

        // Detecção do IPv6
        let url6 = "https://api6.ipify.org";
        let response6 = reqwest::blocking::get(url6)?;

        // Impressão do IPv6
        if response6.status().is_success() {
            let data6 = response6.text()?;
            println!("Seu endereço IPv6 público é: {}", data6);
        } else {
            println!("ERRO: Falha em tentar obter o endereço IPv6.");
            println!("Causas: IPv6 mal configurado no modem ou neste dispositivo, ISP/Operadora não possui suporte a IPv6, IPv6 desabilitado.");
            println!("Posíveis Soluções: Verificar o suporte ao IPv6 com seu provedor de internet ou operadora, verificar as configurações do modem ou dispositivo.");
            println!("Nota: Não ter acesso ao IPv6 não significa que você está offline. Você ainda conseguirá se comunicar com sites/serviços que usam o IPv4, mas não poderá usar ou se comunicar com serviços que usam IPv6.");
            println!("O IPv6 é um protocolo mais recente, então não se preocupe caso não tenha suporte a ele.");
            println!("Saiba mais em: ipv6.org & ipv6.com !");
        }

        // Intervalo de 5 segundos
        thread::sleep(Duration::from_secs(5));

        // Limpa a tela
        clearscreen::clear().unwrap();
       
    
}
}
