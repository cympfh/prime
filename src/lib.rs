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

/// is prime?
fn fermat_test(n: u64) -> Option<bool> {
    if let Ok(q) = u32::try_from(n - 1) {
        for a in 2..1_000_000 {
            if a >= n {
                return None;
            }
            if a.pow(q) != 1 {
                return Some(false);
            }
        }
        Some(true)
    } else {
        None
    }
}

#[wasm_bindgen]
pub fn echo(n: u64) -> u64 {
    n
}

#[wasm_bindgen]
pub fn prime_test(n: u64) -> Option<bool> {
    if n < 2 {
        return Some(false);
    }
    if let Some(result) = trial_division(n) {
        return Some(result);
    }
    if let Some(result) = fermat_test(n) {
        return Some(result);
    }
    return None;
}
