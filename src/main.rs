pub(crate) fn main() {
    println!("Olá");
    println!("Entre com o primeiro número");
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).unwrap();
    let n1: i32 = entrada.trim().parse().unwrap();
    println!("Entre com o segundo número");
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).unwrap();
    let n2: i32 = entrada.trim().parse().unwrap();
    println!("A soma é {}", n1 + n2);
}
