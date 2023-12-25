fn main() {
    repeticao1();
}

fn repeticao1() {
    let multiplicador:u8 = 5;
    let mut contador = 0;

    
    while contador > 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);
    }

}