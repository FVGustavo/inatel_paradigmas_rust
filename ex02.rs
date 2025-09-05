use std::io;

fn eh_par(numero: i32) -> bool {
    numero % 2 == 0
}

fn main() {
    let escolha_j1 = loop {
        println!("Jogador 1, escolha 'par' ou 'ímpar':");
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler entrada");
        let entrada = entrada.trim().to_lowercase();

        if entrada == "par" || entrada == "ímpar" || entrada == "impar" {
            break entrada;
        } else {
            println!("Escolha inválida! Digite apenas 'par' ou 'ímpar'.");
        }
    };

    println!("Jogador 1, digite seu número:");
    let mut numero_j1 = String::new();
    io::stdin()
        .read_line(&mut numero_j1)
        .expect("Erro ao ler número");
    let numero_j1: i32 = numero_j1.trim().parse().expect("Digite um número válido");

    println!("Jogador 2, digite seu número:");
    let mut numero_j2 = String::new();
    io::stdin()
        .read_line(&mut numero_j2)
        .expect("Erro ao ler número");
    let numero_j2: i32 = numero_j2.trim().parse().expect("Digite um número válido");

    let soma = numero_j1 + numero_j2;
    println!("A soma dos números é: {}", soma);

    let soma_par = eh_par(soma);

    if (soma_par && escolha_j1 == "par")
        || (!soma_par && (escolha_j1 == "ímpar" || escolha_j1 == "impar"))
    {
        println!("Jogador 1 venceu!");
    } else {
        println!("Jogador 2 venceu!");
    }
}
