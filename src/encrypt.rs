use std::iter;

static DOUBLE_STEPPING : bool = true;

static ROTORS : [&'static str; 8] = [
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
    "AJDKSIRUXBLHWTMCQGZNPYFVOE",
    "BDFHJLCPRTXVZNYEIWGAKMUSQO",
    "ESOVPZJAYQUIRHXLNFTGKDCMWB",
    "VZBRGITYUPSDNHLXAWMJQOFECK",
    "JPGVOUMFYQBENHZRDKASXLICTW",
    "NZJHGRCXMYSWBOUFAIVLPEKQDT",
    "FKQHTLXOCBJSPDZRAMEWNIUYGV",
];

static TURNOVERS : [&'static str; 8] = [
    "Q",
    "E",
    "V",
    "J",
    "Z",
    "ZM",
    "ZM",
    "ZM",
];

static REFLECTORS : [&'static str; 5] = [
    "EJMZALYXVBWFCRQUONTSPIKHGD",
    "YRUHQSLDPXNGOKMIEBFZCWVJAT",
    "FVPJIAOYEDRZXWGCTKUQSBNMHL",
    "ENKQAUYWJICOPBLMDXZVFTHRGS",
    "RDOBJNTKVEHMLFCWZAXGYIPSUQ",
];

struct Rotor {
    sigma: Vec<u8>,
    sigma_inv: Vec<u8>,
    turnover: u8,
}

struct Config {
    rotors: Vec<Rotor>,
    sigma_reflector: Vec<u8>,
    plugboard: Vec<u8>,
    plugboard_inv: Vec<u8>,
    rings: Vec<u8>,
}

fn ord(c : char) -> Option<u8> {
    if 'a' <= c && c <= 'z' {
        return Some(c as u8 - 'a' as u8);
    }
    else if 'A' <= c && c <= 'Z' {
        return Some(c as u8 - 'A' as u8);
    }
    else { return None; }
}

fn chr(o : u8) -> char {
    if o < 26 {
        return (o + 'A' as u8) as char;
    }
    return '?';
}

fn add26(x : u8, y : u8) -> u8 {
    let res = x + y;
    return if 26 <= res { res - 26 } else { res };
}
fn sub26(x : u8, y : u8) -> u8 { return add26(x, 26 - y); }

fn inv_permutation(sigma : &Vec<u8>) -> Vec<u8> {
    let mut sigma_inv: Vec<u8> = iter::repeat(0u8).take(sigma.len()).collect();
    for (idx, &sigma_idx) in sigma.iter().enumerate() {
        sigma_inv[sigma_idx as usize] = idx as u8;
    }
    return sigma_inv;
}

fn step(state : &mut Vec<u8>, config : &Config) {
    if DOUBLE_STEPPING {
        let mut last_gray = None;
        for idx in (0us .. config.rotors.len() - 1) {
            if state[idx] == config.rotors[idx].turnover { last_gray = Some(idx); }
        }
        match last_gray {
            None => state[0] = add26(state[0], 1),
            Some(last_gray) => {
                for idx in (0us .. last_gray + 2) {
                    state[idx] = add26(state[idx], 1);
                }
            }
        }
    }
    else {
        for (idx, rotor) in config.rotors.iter().enumerate() {
            let should_break = state[idx] != rotor.turnover;
            state[idx] = add26(state[idx], 1);
            if should_break { break; }
        }
    }
}

fn encrypt_one(value : u8, state : &mut Vec<u8>, config : &Config) -> u8 {
    step(state, config);
    let mut value = config.plugboard[value as usize];
    for (idx, rotor) in config.rotors.iter().enumerate() {
        value = add26(sub26(value, config.rings[idx]), state[idx]);
        value = rotor.sigma[value as usize];
        value = sub26(add26(value, config.rings[idx]), state[idx]);
    }
    value = config.sigma_reflector[value as usize];
    for (idx, rotor) in config.rotors.iter().enumerate().rev() {
        value = add26(sub26(value, config.rings[idx]), state[idx]);
        value = rotor.sigma_inv[value as usize];
        value = sub26(add26(value, config.rings[idx]), state[idx]);
    }
    return config.plugboard_inv[value as usize];
}

fn str_to_vec8(input : &str) -> Vec<u8> {
    return input.chars().map(|c| c as u8 - 'A' as u8).collect();
}

fn str_to_vec8_rev(input : &str) -> Vec<u8> {
    return input.chars().map(|c| c as u8 - 'A' as u8).rev().collect();
}


fn plugboard_config(plugboard: &Vec<(char, char)>) -> Vec<u8> {
    let mut id: Vec<u8> = (0..26).map(|x| x as u8).collect();
    for &(c1, c2) in plugboard.iter() {
        let c1 = c1 as u8 - 'A' as u8;
        let c2 = c2 as u8 - 'A' as u8;
        id[c1 as usize] = c2;
        id[c2 as usize] = c1;
    }
    return id;
}

fn create_config(rotor_config : &Vec<usize>, rings : &str) -> Config {
    let mut rotors = Vec::new();
    for &rotor_idx in rotor_config.iter() {
        let sigma = str_to_vec8(ROTORS[rotor_idx]);
        let turnover = TURNOVERS[rotor_idx].chars().next().unwrap() as u8 - 'A' as u8;
        let sigma_inv = inv_permutation(&sigma);
        let rotor = Rotor { sigma: sigma, sigma_inv: sigma_inv, turnover: turnover };
        rotors.push(rotor);
    }
    let sigma_reflector = str_to_vec8(REFLECTORS[1]);
    let plugboard: Vec<(char, char)> = Vec::new();
    let plugboard = plugboard_config(&plugboard);
    let plugboard_inv = inv_permutation(&plugboard);
    return Config {
        rotors: rotors,
        sigma_reflector: sigma_reflector,
        plugboard: plugboard,
        plugboard_inv: plugboard_inv,
        rings: str_to_vec8(rings),
    };
}

pub fn encrypt(input : &str, rotor_config : &Vec<usize>, key : &str, rings : &str) -> String {
    let config = create_config(rotor_config, rings);
    let mut state = str_to_vec8_rev(key);
    return input.chars().filter_map(|c|
        ord(c).map(|c| chr(encrypt_one(c, &mut state, &config)))).collect();
}

fn test_one(plaintext: &str, ciphertext: &str, rotor_config: &Vec<usize>, key: &str, rings: &str) {
    let computed_ciphertext = encrypt(plaintext, rotor_config, key, rings);
    let computed_plaintext = encrypt(ciphertext, rotor_config, key, rings);
    assert_eq!(computed_plaintext, plaintext);
    assert_eq!(computed_ciphertext, ciphertext);
}

#[test]
fn encrypt_tests() {
    test_one(
        "QUEJAIMEAFAIREAPPRENDREUNNOMBREUTILEAUXSAGESIMMORTELARCHIMEDEARTISTEINGENIEURQUIDETONJUGEMENTPEUTPRISERLAVALEURPOURMOITONPROBLEMEEUTDEPAREILSAVANTAGES".as_slice(),
        "UBTSGAGKIOJYHNNGYGWDIEXLIQQHDVALZBFLTKVPIDHNHPETEHGGEEKDCCGBSWDQJGYFPUDHIVBWNLTJHPJPTMHJYFPKSYUBUOPOTFHSJJBFCVUJVJWSMDJVQCZKEMBYLBJFIZRDZFCIQORVGBOBIT".as_slice(),
        &vec![ 0us, 1, 2 ], "AAA".as_slice(), "AAA".as_slice()
    );
    test_one(
        "HELLOWORLD".as_slice(),
        "CDMOGOSHXC".as_slice(),
        &vec![ 0us, 1, 2 ], "LMZ".as_slice(), "AAA".as_slice()
    );
}