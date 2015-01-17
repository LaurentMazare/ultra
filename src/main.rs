use std::io;
use std::os;
mod encrypt;
mod quadgram_data;
mod cryptanalysis;

fn main_encrypt(args: &Vec<String>) {
    if args.len() != 4 {
        println!("Usage: {} encrypt KEY RNG", args[0]);
    }
    else {
        let rotor_config = vec![ 0us, 1, 2 ];
        let key = args[2].as_slice(); 
        let rings = args[3].as_slice();
        if key.len() != 3 {
            println!("Key '{}' has a length different from 3", key);
        }
        else if rings.len() != 3 {
            println!("Rings '{}' has a length different from 3", key);
        }
        else {
            match io::stdin().read_line() {
                Ok(input) => {
                    let output = encrypt::encrypt(input.as_slice(), &rotor_config, key, rings);
                    println!("{}", output);
                },
                Err(_) => ()
            }
        }
    }
}

fn main_break(args: &Vec<String>) {
    match io::stdin().read_line() {
        Ok(input) => {
            match cryptanalysis::brute_force(input.as_slice()) {
                None => println!("No optimal key found."),
                Some((score, key, rotor_config, rings)) => {
                    println!("{} {}", key, score);
                    println!("{}", encrypt::encrypt(input.as_slice(), &rotor_config, key.as_slice(), rings.as_slice()));
                }
            }
        },
        Err(_) => ()
    }
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: {} encrypt|break", args[0]);
        return;
    }
    match args[1].as_slice() {
        "encrypt" => main_encrypt(&args),
        "break" => main_break(&args),
        otherwise => println!("Unrecognized argument {}, use encrypt or break", otherwise),
    }
}

