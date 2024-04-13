# Teste de conexão e Latencia 
Escrito em RUST
Autor: Edival Mittelstad

## Compilação
cargo build --release

### Opções 
    -S                  Inicializa como servidor
    -i IP               IP do servidor para conectar
    -p PORTA            Porta de conexão
    -e                  Exibir Latencia
    -g                  Gravar Latencia
    -t SEGUNDOS         Intervalo de teste em segundos (Padrão 3)
    --help              Exibe esta mensagem de ajuda

### Exemplos

#### Iniciar o servidor na porta 8000
$ ./connect -S -p 8000

#### Iniciar o cliente com intervalo de teste de 1 segundo e exibindo a latencia
$ ./connect -i 127.0.0.1 -p 8000 -t 1 -e


