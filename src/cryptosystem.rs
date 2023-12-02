use std::ops::{Add, Mul};
use malachite::{Integer, Natural};
use malachite::num::arithmetic::traits::{JacobiSymbol, Mod, ModInverse, ModPow, Pow, Square};
use malachite::num::basic::traits::One;
use malachite::num::logic::traits::BitIterable;

pub fn format(m: &Natural, n: &Natural, r: &Natural) -> Option<Natural> {
    let l = n.bits().count().div_ceil(8usize);
    if m.bits().count().div_ceil(8usize) > l - 10 {
        return None;
    }
    let x = Natural::from(255u16) * Natural::from(2u8).pow(8 * (l as u64 - 2)) + m * Natural::from(2u8).pow(64) + r;

    Some(x)
}

// pub fn generate(p: &Natural, q: &Natural) -> (Natural){
//     let n = p*q;
//     let oiler = (p - Natural::ONE) * (q - Natural::ONE);
//     let e = Natural::from(0b10000000000000001u64);
//     let d = e.mod_inverse(oiler).unwrap();
//     (d)
// }

pub fn encrypt(x: &Natural, n: &Natural) -> (Natural, Natural, i8) { // x - message
    let b = Natural::from(2u8).pow(128u64);
    let y = x.mul(x.add(&b)).mod_op(n);
    let c1 = (x + &b / Natural::from(2u8)).mod_op(n).mod_op(Natural::from(2u8));
    let c2 = (x + &b / Natural::from(2u8)).jacobi_symbol(n);
    (y, c1, c2)
}

pub fn decrypt(y: &Natural, c1: &Natural, n: &Natural, c2: i8) {
    let b = Natural::from(2u8).pow(128u64);
    let x = (-(&b/Natural::from(2u8)) + Integer::from(y+ b.pow(2u64)/Natural::from(4u8))).square().mod_op(Integer::from(n));
}