use std::io::{self, Write}; //std::io::{self, Write};	Importa entrada e saída (teclado/tela)


// loop	Repetir até o usuário digitar algo válido
// match	Tratar sucesso (Ok) ou erro (Err)
// Ok(_)	Sabemos que funcionou, mas não usamos o valor retornado
// Err(_)	Mostrar erro se a leitura falhar
// .is_empty()	Impedir entradas vazias ou espaços em branco
// .expect()	Mostrar mensagem clara se o .flush() der erro

fn main () {
    let nome = ler_texto("Qual o seu nome? ");
    println!("Meu nome é: {}", nome);

    let profissao = ler_texto("Qual a sua profissao? ");
    println!("Minha profissão é: {}", profissao);

    let idade = ler_idade("Qual sua idade? ");
    println!("Minha idade é: {} anos ", idade);
}

fn ler_texto (menssagem: &str) -> String {
    loop {
        print!("{}", menssagem);
        //o .expect() mostrar o erro de forma mais descritivar do que o unwrap que encerra o programa.
        io::stdout().flush().expect("Erro ao forçar saida no terminal");

        let mut entrada = String::new();

        // Tenta ler a linha. Se falhar, avisa o usuario e repete o loop
        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {
                let texto = entrada.trim();
                if texto.is_empty() {
                    println!("Entrada vazia, tente novamente")
                }else {
                    return texto.to_string();
                }
            }
            Err(_) => {
                println!("Ocorreu um erro ao tentar ler sua entrada, tente novamente!")
            }
        }
    }
}

fn ler_idade(menssagem: &str) -> i32 {
    loop {
        print!("{}", menssagem);
        io::stdout().flush().expect("Erro ao forçar saida no terminal");

        let mut entrada = String::new();

        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {
                let numero = entrada.trim();
                if numero.is_empty() {
                    println!("Numero invalido ou vazio, tente novamente!")
                }else {
                    match numero.parse::<i32>() {
                        Ok(valor) => return valor,
                        Err(_) => println!("Digite um numero inteiro valido!")
                    }
                }
            }
            Err(_) => {
                println!("Erro ao tentar ler a idade, tente novamente!")
            }
        }
    }
}