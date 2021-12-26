use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::{self, Colorize};
fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;
    println!("¡Adivina el número! Introduce a continuación un número entre 1 y 100 (ambos incluidos):");
    
    loop {
        tries += 1;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("No se pudo leer la entrada.");

        let input : u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        }; 

        match input.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}, lo conseguiste en {} intentos", "¡Correcto!".green(), tries);
                break;
            }
            Ordering::Greater => {
                println!("{}", "Demasiado grande".red())
            }
            Ordering::Less => {
                println!("{}", "Demasiado pequeño".red())
            }
        }
    }
    
}
