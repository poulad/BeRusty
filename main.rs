fn print_stuff() {
    eprintln!("{greeting} {to}", greeting = "hola", to = "mundo");

    let output: String = format!("{greeting} {to}", greeting = "hola", to = "mundo");
    println!("{}", output);

    let pi = 3.141592;
    println!("Pi is roughly {value:.3}", value=pi)
}

fn main() {
    print_stuff()
}