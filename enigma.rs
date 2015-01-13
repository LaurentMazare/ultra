use std::io;
use std::iter;
use std::os;
mod encrypt;

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("Usage: {} KEY", args[0]);
    }
    else {
        let key = args[1].as_slice(); 
        if key.len() != 3 {
            println!("Key '{}' has a length different from 3", key);
        }
        else {
            match io::stdin().read_line() {
                Ok(input) => {
                    let output = encrypt::encrypt(input.as_slice(), key);
                    println!("{}", output);
                },
                Err(_) => ()
            }
        }
    }
}
