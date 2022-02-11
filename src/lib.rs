use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn primes(n: usize) -> Vec<usize> {
    let mut arr = vec![true; n.try_into().unwrap()];
    let mut primes = vec![];
    let limit = (n as f32).sqrt() as usize;

    for i in 2..limit {
        if arr[i] {
            let mut j = i * i;
            while j < n {
                arr[j] = false;
                j = j + i;
            }
        }
    }

    for i in 2..n {
        if arr[i] {
            primes.push(i);
        }
    }

    primes
}
