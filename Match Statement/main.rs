fn main() {
    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "DS",
        _ => ""
    }
    println!("O proposito de {} é {}", linguagem, proposito)
}