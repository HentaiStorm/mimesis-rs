use std::cmp::PartialOrd;

use itertools::Itertools;

use rand::prelude::*;


/// Get random bigint in range from a to b
/// 
/// # Arguments
/// * `a` - Minimum value of range
/// * `b` - Maximum value of range
pub fn randbigint(a: u128, b: u128) -> num::BigUint {
    num::BigUint::from_bytes_be(&randint(a, b).to_be_bytes())
}

/// Get random int in range from a to b
/// 
/// # Arguments
/// * `a` - Minimum value of range
/// * `b` - Maximum value of range
pub fn randint<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd>(a: T, b: T) -> T {
    StdRng::from_entropy().gen_range(a..=b)
}

/// Get a bool with a probability p of being true.
pub fn rand_bool(p: f64) -> bool {
    StdRng::from_entropy().gen_bool(p)
}

/// Generate vec of random int
/// 
/// # Arguments
/// * `amount` - Amount of elements
/// * `a` - Minimum value of range
/// * `b` - Maximum value of range
pub fn randints<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd + Copy>(a: T, b: T, amount: usize) -> Vec<T> {
    (0..amount).map(|_| randint(a, b)).collect()
}

/// Return a bytes object containing random bytes
/// 
/// # Arguments
/// * `size` - The size of u8 vector
pub fn urandom(size: usize) -> Vec<u8> {
    (0..size).map(|_| randint(u8::MIN, u8::MAX)).collect()
}

/// Generate random string created from string sequence
/// 
/// # Arguments
/// * `str_seq` - String sequence of letters or digits
/// * `length` - Max value
pub fn generate_string(str_seq: &str, length: usize) -> String {
    (0..length).map(|_| get_random_element(str_seq.chars()).to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Generate custom code using ascii uppercase and random integers
///
/// e.g mask = "@###", char = '@', digit = '#')
/// 
/// # Arguments
/// * `mask` - Mask of code
/// * `char` - Placeholder for characters
/// * `digit` - Placeholder for digits
pub fn custom_code(mask: &str, char: char, digit: char) -> String {
    mask.chars().map(|c| {
        if c == char {
            get_random_element("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()).to_string()
        } else if c == digit {
            randint(0, 9).to_string()
        } else {
            c.to_string()
        }
    }).join("")
}


/// Get f32 in range from a to b
/// 
/// # Arguments
/// * `a` - Minimum value of range
/// * `b` - Maximum value of range
pub fn uniform(a: f32, b: f32) -> f32 {
    rand::random::<f32>() * (b - a) + a
}


/// Generate random string value
/// 
/// This method can be especially useful when you need to generate
///  only unique values in your provider. Just pass parameter unique=True.
/// 
/// # Arguments
/// * `unique` - Generate only unique values base on uuid4
/// * `length` - Length of string, does not affect the result with unique bool
pub fn randstr(unique: bool, length: usize) -> String {
    if unique {
        uuid::Uuid::new_v4().to_string()
    } else {
        get_random_elements("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(), length)
            .iter().map(|c| c.to_string()).join("")
    }
}


/// Get random element from random iterator
/// 
/// *Use clear and uniterated arg*
/// 
/// # Arguments
/// * `iter` - Iterator for choose random element
pub fn get_random_element<T, V: Iterator<Item = T>>(iter: V) -> T {
    iter.choose(&mut rand::thread_rng()).unwrap()
}

/// Get random elements from random iterator
/// 
/// *Use clear and uniterated arg*
/// 
/// # Arguments
/// * `iter` - Iterator for choose random element
/// * `quantity` - Quantity of iterator
pub fn get_random_elements<T, V: Iterator<Item = T> + Clone>(iter: V, quantity: usize) -> Vec<T> {
    (0..quantity).map(|_| iter.clone().choose(&mut rand::thread_rng()).unwrap()).collect()
}


/// Get enum value if is not None, else default or random value
pub fn validate_enum<T, E: valued_enums::ValuedEnum<T>>(e: Option<E>, default: Option<E>) -> T {
    match e {
        Some(x) => x.value(),
        None => match default {
            Some(d) => d.value(),
            None => get_random_element(E::values().into_iter()),
        },
    }
}

/// Get enum value if is not None, else default or random value
pub fn validate_variant<T, E: valued_enums::ValuedEnum<T>>(e: Option<E>, default: Option<E>) -> E {
    match e {
        Some(x) => x,
        None => match default {
            Some(d) => d,
            None => get_random_element(E::variants().into_iter()),
        },
    }
}
