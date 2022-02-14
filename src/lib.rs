use wasm_bindgen::prelude::*;

/// is prime?
fn trial_division(n: u64) -> Option<bool> {
    for i in 2..1_000_000 {
        if i * i > n {
            return Some(true);
        }
        if n % i == 0 {
            return Some(false);
        }
    }
    None
}

#[wasm_bindgen]
pub fn prime_test(n: u64) -> Option<bool> {
    if n < 2 {
        return Some(false);
    }
    if let Some(result) = trial_division(n) {
        return Some(result);
    }
    return None;
}
