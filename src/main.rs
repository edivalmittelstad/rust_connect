use std::env;
extern crate getopts;
use getopts::Options;

mod client;
mod server;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Uso: {} [opções]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("S", "", "Inicializa como servidor");
  opts.optopt("i", "", "IP do servidor para conectar - (Cliente)", "IP");
  opts.optopt("p", "", "Porta de conexão", "PORTA");
  opts.optflag("e", "", "Exibir Latencia - (Cliente)");
  opts.optflag("g", "", "Gravar Latencia - (Cliente)");
  opts.optopt("t", "", "Intervalo de teste em segundos (Padrão 3) - (Cliente)", "SEGUNDOS");
  opts.optflag("", "help", "Exibe esta mensagem de ajuda");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { 
        println!("{}", f.to_string());
        print_usage(&program, opts);
        return;
    }
  };

  if matches.opt_present("help") {
    print_usage(&program, opts);
    return;
  }

  if !matches.opt_present("p") {
    println!("A opção -p (porta) é obrigatória");
    print_usage(&program, opts);
    return;
}

  let mut is_server: bool = false;
  let show_lat: bool = matches.opt_present("e");
  let write_lat: bool = matches.opt_present("g");

  let porta: i32 = match matches.opt_str("p") {
    Some(p) => match p.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Erro: A porta deve ser um número inteiro.");
        print_usage(&program, opts);
        return;
      }
    },
    None => {
      println!("A opção -p (porta) é obrigatória");
      print_usage(&program, opts);
      return;
    }
  };

  let intervalo: u64 = match matches.opt_str("t") {
    Some(t) => match t.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Erro: A intervalo deve ser um número inteiro.");
        print_usage(&program, opts);
        return;
      }
    },
    None => {
      3
    }
  };


  if matches.opt_present("S") {
    is_server = true;
  }

  if is_server {
    server::connect_server(porta);
    return;
  } 

  if let Some(ip) = matches.opt_str("i") {
    client::connect_client(ip, porta, show_lat, write_lat, intervalo);
    return;
  } 
  print_usage(&program, opts);

}
