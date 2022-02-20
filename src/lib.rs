extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;

fn powmod(a: u64, k: u64, modulo: u64) -> u64 {
    if k == 0 {
        1
    } else {
        let mut z = powmod(a, k / 2, modulo) as u128;
        let modulo = modulo as u128;
        let a = a as u128;
        z = (z * z) % modulo;
        if k % 2 == 1 {
            z = (z * a) % modulo;
        }
        z as u64
    }
}

fn trivial_primes(n: u64) -> Option<bool> {
    if n == 2 {
        Some(true)
    } else if n < 2 || n % 2 == 0 {
        Some(false)
    } else {
        None
    }
}

/// is prime?
fn trial_division(n: u64) -> Option<bool> {
    if let Some(res) = trivial_primes(n) {
        return Some(res);
    }
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
fn miller_rabin(n: u64) -> Option<bool> {
    if let Some(res) = trivial_primes(n) {
        return Some(res);
    }
    /// composite condition
    fn sub(n: u64, a: u64) -> bool {
        let mut s = 0;
        let mut d = n - 1;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        let mut x = powmod(a, d, n);
        if x == 1 {
            return false;
        }
        for _ in 0..s {
            if x == n - 1 {
                return false;
            }
            x = powmod(x, 2, n);
        }
        true
    }

    for a in 2..32.min(n) {
        if sub(n, a) {
            return Some(false);
        }
    }
    Some(true)
}

#[wasm_bindgen]
pub fn echo(n: u64) -> u64 {
    n
}

#[wasm_bindgen]
pub fn prime_test(n: u64) -> Option<bool> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    if n < 2 {
        return Some(false);
    }
    if let Some(result) = trial_division(n) {
        return Some(result);
    }
    if let Some(result) = miller_rabin(n) {
        return Some(result);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_powmod() {
        assert_eq!(powmod(2, 10, 1000), 24);
        assert_eq!(powmod(2, 20, 127), 64);
    }
    #[test]
    fn test_trial_division() {
        assert_eq!(trial_division(2), Some(true));
        assert_eq!(trial_division(3), Some(true));
        assert_eq!(trial_division(4), Some(false));
        assert_eq!(trial_division(6), Some(false));
        assert_eq!(trial_division(127), Some(true));
        assert_eq!(trial_division(67_280_421_310_721), None);
    }
    #[test]
    fn test_miller_rabin() {
        assert_eq!(miller_rabin(67_280_421_310_721), Some(true));
        for p in 1..1_000_000 {
            assert_eq!(trial_division(p), miller_rabin(p), "p={}", p);
        }
    }
}
