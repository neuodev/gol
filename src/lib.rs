mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn fibo(idx: i64) -> i64 {
    if idx < 2 {
        idx
    } else {
        fibo(idx - 2) + fibo(idx - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::fibo;

    #[test]
    fn get_fibo_seq() {
        let nums: [i64; 10] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        nums.iter().enumerate().for_each(|(idx, n)| {
            assert_eq!(fibo(idx as i64), *n);
        });
    }
}
