use crate::types::{MAX_VALUE, MIN_VALUE, Trit, WORD_SIZE, Word};

const BIAS: i32 = MAX_VALUE;

pub fn convert_int_to_unbalanced(value: i32) -> Vec<i8> {
    let mut trits: Vec<i8> = Vec::new();
    let mut biased = value + BIAS;

    while biased != 0 {
        trits.push((biased % 3) as i8);
        biased /= 3;
    }
    for i in trits.len()..WORD_SIZE {
        trits.push(0);
    }
    trits.reverse(); // take the 0th index as the most significant trit
    trits
}

pub fn convert_unbalanced_to_word(unbalanced: &[i8]) -> Word {
    if unbalanced.len() > WORD_SIZE {
        panic!(
            "Unbalanced trit vector too long: {}. Must be at most {}.",
            unbalanced.len(),
            WORD_SIZE
        );
    }
    let mut word: Word = Word::new();
    for (i, item) in unbalanced.iter().enumerate() {
        word.set_trit(i, &Trit::state(item - 1));
    }
    for i in unbalanced.len()..WORD_SIZE {
        word.set_trit(i, &Trit::N);
    }
    word
}

pub fn convert_int_to_word(value: i32) -> Word {
    if !(MIN_VALUE..=MAX_VALUE).contains(&value) {
        panic!(
            "Value out of range: {}. Must be between {} and {}.",
            value, MIN_VALUE, MAX_VALUE
        );
    }
    convert_unbalanced_to_word(&convert_int_to_unbalanced(value))
}

pub fn convert_word_to_int(word: &Word) -> i32 {
    let trits = word.get_trits();
    let mut value = 0;
    for i in trits.iter() {
        value *= 3;
        match i {
            Trit::Z => {}
            Trit::P => value += 1,
            Trit::N => value -= 1,
        }
    }
    value
}

pub fn convert_int_to_unbiased_unbalanced(value: i32) -> Vec<i8> {
    let mut trits: Vec<i8> = Vec::new();
    let mut val = value;

    while val != 0 {
        trits.push((val % 3) as i8);
        val /= 3;
    }
    for i in trits.len()..WORD_SIZE {
        trits.push(0);
    }
    trits.reverse(); // take the 0th index as the most significant trit
    trits
}

pub fn convert_int_to_unsigned_word(value: i32) -> Word {
    if !(0..=MAX_VALUE).contains(&value) {
        panic!(
            "Value out of range: {}. Must be between 0 and {}.",
            value,
            MAX_VALUE * 2
        );
    }
    convert_unbalanced_to_word(&convert_int_to_unbiased_unbalanced(value))
}
