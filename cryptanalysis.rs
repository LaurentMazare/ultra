use std::io;
use std::collections::BTreeSet;
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

fn get_worst(treeset: &BTreeSet<(i64, String, Vec<usize>)>) -> Option<(i64, String, Vec<usize>)> {
    match treeset.iter().next().clone() {
        None => None,
        Some(v) => Some(v.clone()),
    }
}

fn brute_force_rotors_and_key(ciphertext : &str, rings : &str) -> BTreeSet<(i64, String, Vec<usize>)> {
    let mut best_rotors_and_key = BTreeSet::new();
    // Quite awful and inefficient...
    for rotor_config in Product::new(5us, 3us) {
        if rotor_config[0] == rotor_config[1] ||
           rotor_config[0] == rotor_config[2] ||
           rotor_config[1] == rotor_config[2] { continue; }
        for cs in Product::new(26us, 3us) {
            let key : String = cs.iter().map(|&x| chr(x as u8)).collect();
            let plaintext = encrypt::encrypt(ciphertext.as_slice(), &rotor_config, key.as_slice(), rings);
            let score = score(plaintext.as_slice());
            let score = score as i64;
            // Only keep the 100 best keys...
            if best_rotors_and_key.len() < 100 {
                best_rotors_and_key.insert((score, key, rotor_config.clone()));
            }
            else {
                match get_worst(&best_rotors_and_key) {
                    None => (),
                    Some(worst) => {
                        let (worst_score, _, _) = worst;
                        if worst_score < score {
                            best_rotors_and_key.remove(&worst);
                            best_rotors_and_key.insert((score, key, rotor_config.clone()));
                        }
                    },
                }
            }
        }
    }
    return best_rotors_and_key;
}

fn brute_force(ciphertext : &str) -> Option<(f64, String, Vec<usize>, String)> {
    let mut maximum_score = 0. as f64;
    let mut where_max = None;
    let best_rotors_and_key = brute_force_rotors_and_key(ciphertext, "AAA".as_slice());
    for &(affinity, ref key, ref rotor_config) in best_rotors_and_key.iter().rev() {
        let rotor_config_str: String = rotor_config.iter().map(|&x| (x as u8 + '0' as u8) as char).collect();
        for cs in Product::new(26us, 3us) {
            let rings : String = cs.iter().map(|&x| chr(x as u8)).collect();
            let key : String = key.chars().zip(cs.iter()).map(|(x, &y)| chr((ord(x) + y as u8) % 26)).collect();
            let plaintext = encrypt::encrypt(ciphertext, rotor_config, key.as_slice(), rings.as_slice());
            let s = score(plaintext.as_slice());
            if maximum_score == 0. || maximum_score < s {
                maximum_score = s;
                println!(">>> {} {} {} {}", s, key, rings, plaintext);
                where_max = Some((s, key, rotor_config.clone(), rings));
            }
        }
    }
    return where_max;
}

fn main() {
    match io::stdin().read_line() {
        Ok(input) => {
            match brute_force(input.as_slice()) {
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

