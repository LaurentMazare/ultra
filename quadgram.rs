use std::io;
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
