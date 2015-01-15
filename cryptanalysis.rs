use std::io;
mod encrypt;
mod quadgram_data;

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
    for c1 in (0u8 .. 26) {
        for c2 in (0u8 .. 26) {
            for c3 in (0u8 .. 26) {
                // This is likely to be very inefficient.
                let key : String = [ chr(c1), chr(c2), chr(c3) ].iter().map(|x| *x).collect();
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
        }
    }
    return maximum_score;
}

fn brute_force_rings(ciphertext : &str, rings : &str) -> Option<(f64, Vec<usize>, String)> {
    let mut maximum_score : Option<(f64, Vec<usize>, String)> = None;
    // Quite awful and inefficient...
    for r1 in (0us .. 5) {
        for r2 in (0us .. 5) {
            for r3 in (0us .. 5) {
                if r1 == r2 || r1 == r3 || r2 == r3 { continue; }
                let rotor_config = vec![ r1, r2, r3 ];
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
            let key_vec : Vec<char> = key.chars().collect();
            let k1 = ord(key_vec[0]);
            let k2 = ord(key_vec[1]);
            let k3 = ord(key_vec[2]);
            for c1 in (0u8 .. 26) {
                for c2 in (0u8 .. 26) {
                    for c3 in (0u8 .. 26) {
                        let rings : String = [ chr(c1), chr(c2), chr(c3) ].iter().map(|x| *x).collect();
                        let key : String = [ chr((c1 + k1) % 26), chr((c2 + k2 - 1) % 26), chr((c3 + k3) % 26) ].iter().map(|x| *x).collect();
                        let plaintext = encrypt::encrypt(ciphertext, &rotor_config, key.as_slice(), rings.as_slice());
                        let s = score(plaintext.as_slice());
                        println!("{} {} {} {}", rings, key, plaintext, s);
                        if maximum_score < s {
                            maximum_score = s;
                            where_max = rings;
                        }
                    }
                }
            }
            return Some((maximum_score, rotor_config, key, where_max));
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

