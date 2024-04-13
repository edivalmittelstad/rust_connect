use std::net::TcpListener;
use std::io::{Read, Write};

pub fn connect_server(port: i32 ) {
  let address = format!("0.0.0.0:{}", port);
  let listener = TcpListener::bind(address).expect("Falha ao vincular o socket à porta");

  println!("Escutando na porta {}", port);
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let addr = stream.peer_addr().expect("Falha ao obter o endereço do cliente");
        println!("Origem: {}", addr);
        let mut buffer = [0; 1];
        loop {
          match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
              stream.write(b"1").expect("Falha ao enviar dados para o cliente");
            }
            Err(_) => break,
          }
        }
      }
      Err(e) => {
        eprintln!("Erro ao aceitar a conexão: {}", e);
      }
    }
  }
}