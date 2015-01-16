use std::io;
mod encrypt;
mod quadgram_data;

struct Product {
    state: Vec<usize>,
    max_value: usize,
}

impl Product {
    fn new(max_value: usize, n: usize) -> Product {
        let mut state = Vec::new();
        state.resize(n, 0us);
        return Product { max_value: max_value, state: state };
    }
}

impl Iterator for Product {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let res = self.state.clone();
        let n = self.state.len();
        for i in (0us .. n) {
            let i = n - 1 - i;
            if self.state[i] != self.max_value - 1 {
                self.state[i] += 1;
                return Some(res);
            }
            else {
                self.state[i] = 0us;
            }
        }
        return None;
    }
}

fn score(text : &str) -> f64 {
    let mut score : f64 = 0.0;
    let mut qgram_index : usize = 0;
    for (idx, c) in text.char_indices() {
        let c = c as i64 - 'A' as i64;
        if c < 0 || 25 < c { continue; }
        qgram_index = (qgram_index % (26 * 26 * 26)) * 26 + c as usize;
        if 3 <= idx {
            score += quadgram_data::QGRAM[qgram_index];
        }
    }
    return score;
}

fn chr(v : u8) -> char {
    return (v + 'A' as u8) as char;
}

fn ord(c : char) -> u8 {
    return (c as u8 - 'A' as u8);
}

fn brute_force_key(ciphertext : &str, rotor_config : &Vec<usize>, rings : &str) -> Option<(f64, String)> {
    let mut maximum_score : Option<(f64, String)> = None;
    for cs in Product::new(26us, 3us) {
        // This is likely to be very inefficient.
        let key : String = cs.iter().map(|&x| chr(x as u8)).collect();
        let plaintext = encrypt::encrypt(ciphertext.as_slice(), rotor_config, key.as_slice(), rings);
        let s = score(plaintext.as_slice());
        let optimal =
            match maximum_score {
                None => true,
                Some((ms, _)) => ms < s
            };
        if optimal {
            maximum_score = Some((s, key));
        }
    }
    return maximum_score;
}

fn brute_force_rings(ciphertext : &str, rings : &str) -> Option<(f64, Vec<usize>, String)> {
    let mut maximum_score : Option<(f64, Vec<usize>, String)> = None;
    // Quite awful and inefficient...
    for rotor_config in Product::new(5us, 3us) {
        if rotor_config[0] == rotor_config[1] ||
           rotor_config[0] == rotor_config[2] ||
           rotor_config[1] == rotor_config[2] { continue; }
        match brute_force_key(ciphertext, &rotor_config, rings) {
            None => (),
            Some ((s, key)) => {
                let optimal =
                    match maximum_score {
                        None => true,
                        Some((ms, _, _)) => ms < s
                    };
                if optimal {
                    maximum_score = Some((s, rotor_config, key));
                }
            }
        }
    }
    return maximum_score;
}

fn brute_force(ciphertext : &str) -> Option<(f64, Vec<usize>, String, String)> {
    let mut where_max = String::from_str("AAA");
    match brute_force_rings(ciphertext, where_max.as_slice()) {
        None => None,
        Some((affinity, rotor_config, key)) => {
            let mut maximum_score = affinity;
            for cs in Product::new(26us, 3us) {
                let rings : String = cs.iter().map(|&x| chr(x as u8)).collect();
                let key : String = key.chars().zip(cs.iter()).map(|(x, &y)| chr((ord(x) + y as u8) % 26)).collect();
                let plaintext = encrypt::encrypt(ciphertext, &rotor_config, key.as_slice(), rings.as_slice());
                let s = score(plaintext.as_slice());
                println!("{} {} {} {}", rings, key, plaintext, s);
                if maximum_score < s {
                    maximum_score = s;
                    where_max = rings;
                }
            }
            Some((maximum_score, rotor_config, key, where_max))
        }
    }
}

fn main() {
    match io::stdin().read_line() {
        Ok(input) => {
            match brute_force(input.as_slice()) {
                None => println!("No optimal key found."),
                Some((score, rotor_config, key, rings)) => {
                    println!("{} {}", key, score);
                    println!("{}", encrypt::encrypt(input.as_slice(), &rotor_config, key.as_slice(), rings.as_slice()));
                }
            }
        },
        Err(_) => ()
    }
}

