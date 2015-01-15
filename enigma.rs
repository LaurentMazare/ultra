use std::io;
use std::os;
mod encrypt;

fn main() {
    let args = os::args();
    if args.len() != 3 {
        println!("Usage: {} KEY RNG", args[0]);
    }
    else {
        let rotor_config = vec![ 2us, 1, 0 ];
        let key = args[1].as_slice(); 
        let rings = args[2].as_slice();
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
