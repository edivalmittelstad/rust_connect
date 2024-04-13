use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;

fn gravar_mudanca(texto: &str) {
  if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("test_connect.log") {
    if let Err(e) = writeln!(file, "{}", texto) {
      eprintln!("Erro ao gravar no arquivo de log: {}", e);
    }
    println!("{}", texto);
  } else {
    eprintln!("Não foi possível abrir o arquivo de log");
  }
}

fn gravar_latencia(texto: &str) {
  if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("latencia_connect.log") {
    if let Err(e) = writeln!(file, "{}", texto) {
      eprintln!("Erro ao gravar no arquivo de latencia: {}", e);
    }
  } else {
    eprintln!("Não foi possível abrir o arquivo de latencia");
  }
}

fn ver_status(atual: i32, novo: i32) -> i32 {
  let dt_string = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");
  if atual != novo {
    if novo < 1 {
      let msg = format!("Falha: {}", dt_string);
      gravar_mudanca(&msg);
      return 0;
    }
    let msg = format!("OK: {}", dt_string);
    gravar_mudanca(&msg);
    return 1;
  }
  atual
}

pub fn connect_client(host: String, port: i32, show_lat: bool, write_lat: bool, intervalo: u64) {
  let mut atual = -10;
  let mut novo;
  eprintln!("Intervalo de teste: {}s", intervalo);
  loop {
    match TcpStream::connect(format!("{}:{}", host, port)) {
      Ok(mut stream) => {
        let mut buffer = [0; 1];
        loop {
          let inicio = Instant::now();
          if let Err(_) = stream.write(b"1") {
            novo = 0;
            break;
          }
          match stream.read(&mut buffer) {
            Ok(_) => {
              let fim = Instant::now();
              let tempo = fim.duration_since(inicio).as_secs_f64() * 1000.0;
              if show_lat {
                println!("Latência de: {:.4}ms", tempo);
              }
              if write_lat {
                gravar_latencia(tempo.to_string().as_str())
              }
              novo = 1;
            }
            Err(_) => {
              novo = 0;
              break;
            }
          }
          if atual != 1 {
            atual = ver_status(atual, novo);
          }
          std::thread::sleep(Duration::from_secs(intervalo));
        }
      }
      Err(_) => novo = 0,
    }
    atual = ver_status(atual, novo);
    std::thread::sleep(Duration::from_secs(intervalo));
  }
}