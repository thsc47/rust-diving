fn main() {
    repeticao1();
    repeticao2();
}

fn repeticao1() {
    let multiplicador:u8 = 5;
    let mut contador = 0;


    while contador > 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);
    }
}

fn repeticao2() {
        let multiplicador:u8 = 5;
    
        for i in 10  {
            println!("{} x {} = {}", multiplicador, i, multiplicador*i);
        }    
    }
