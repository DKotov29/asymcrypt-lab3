use std::ops::{Add, Div, Mul, Rem};
use malachite::{Integer, Natural};
use malachite::num::arithmetic::traits::{ExtendedGcd, JacobiSymbol, Mod, ModInverse, ModPow, Pow, Square};
use malachite::num::basic::traits::{One, Two, Zero};
use malachite::num::logic::traits::BitIterable;
use num_bigint::BigUint;
use num_traits::Num;

pub fn format(m: &Natural, n: &Natural, r: &Natural) -> Option<Natural> {
    let l = n.bits().count().div_ceil(8usize);
    if m.bits().count().div_ceil(8usize) > l - 10 {
        return None;
    }
    let x = (Natural::from(255u16) * (Natural::from(2u8).pow(8 * (l as u64 - 2)))) + (m * (Natural::from(2u8).pow(64))) + r;

    Some(x)
}

// pub fn generate(p: &Natural, q: &Natural) -> (Natural){
//     let n = p*q;
//     let oiler = (p - Natural::ONE) * (q - Natural::ONE);
//     let e = Natural::from(0b10000000000000001u64);
//     let d = e.mod_inverse(oiler).unwrap();
//     (d)
// }

pub fn encrypt(b: &Natural, x: &Natural, n: &Natural) -> (Natural, Natural, i8) { // x - message
    let y = x.mul(x+b).mod_op(n);
    let c1 = (x + b * (Natural::from(2u8).mod_inverse(n).unwrap())).mod_op(n).mod_op(Natural::from(2u8));
    let c2 = (x + (b / Natural::from(2u8))).jacobi_symbol(n);
    (y, c1, c2)
}


pub fn decrypt(b: &Natural, y: &Natural, p: &Natural, q: &Natural, c1: &Natural, n: &Natural, c2: i8) -> Integer {
    // let x = (-(&b / Natural::from(2u8)) + Integer::from(y + b.pow(2u64) / Natural::from(4u8))).square().mod_op(Integer::from(n));
    let x = blum(&(y + (b.pow(2u64) * ((Natural::from(4u8)).mod_inverse(n)).unwrap())).mod_op(n), p, q);
    println!("decr {y}");
    println!("decr with such c1,c2: {c1} {c2}");
    let x = {
        let mut m = None;
        for xx in x {
            let nn = Integer::from(n);
            let xx = xx.mod_op(nn);
            let c11 = (&xx + Integer::from(b) * (Integer::from(&Natural::from(2u8).mod_inverse(n).unwrap())))
                .mod_op(Integer::from(n)).mod_op(Integer::from(2u8));
            let c22 = (&xx +( Integer::from(b / &Integer::from(2)))).jacobi_symbol(Integer::from(n));
            println!("c11,c22,x: {c11} {c22} {}", &xx);

            if c11 == Integer::from(c1) && c22 == c2 {
                m = Some(xx);
                // break;
            }
        }
        if m.is_none() {
            unreachable!("decrypt");
        } else {
            m.unwrap()
        }
    };
    let x = (x - Integer::from(b) * Integer::from(Natural::TWO.mod_inverse(n).unwrap())).mod_op(Integer::from(n));
    x
}

pub fn blum(y: &Natural, p: &Natural, q: &Natural) -> [Integer; 4] {
    // let s1 = y.mod_pow((p + Natural::ONE) / Natural::from(4u8), p);
    // let s2 = y.mod_pow((q + Natural::ONE) / Natural::from(4u8), q);
    let n = p*q;

    let s1 = Natural::from_owned_limbs_asc(BigUint::from_str_radix(format!("{y}").as_str(), 10u32).unwrap().modpow(
        &(BigUint::from_str_radix(format!("{p}").as_str(), 10u32).unwrap() + &BigUint::from(1u8)).div(&BigUint::from(4u8)),
        &BigUint::from_str_radix(format!("{p}").as_str(), 10u32).unwrap()
    ).to_u64_digits());
    let s2 = Natural::from_owned_limbs_asc(BigUint::from_str_radix(format!("{y}").as_str(), 10u32).unwrap().modpow(
        &(BigUint::from_str_radix(format!("{q}").as_str(), 10u32).unwrap() + &BigUint::from(1u8)).div(&BigUint::from(4u8)),
        &BigUint::from_str_radix(format!("{q}").as_str(), 10u32).unwrap()
    ).to_u64_digits());

    let (gcd, u, v) = p.extended_gcd(q);
    if gcd == Natural::ONE {
        let x1 = ((&u * Integer::from(p) * Integer::from(&s1)) - (&v * Integer::from(q) * Integer::from(&s2))).mod_op(Integer::from(&n));
        let x2 = ((&u * Integer::from(p) * Integer::from(&s1)) + (&v * Integer::from(q) * Integer::from(&s2))).mod_op(Integer::from(&n));
        let x3 = Integer::from(&n) - &x1;
        let x4 = Integer::from(&n) - &x2 ;
        println!("x1: {x1}\nx2: {x2}\nx3: {x3}\nx4: {x4}");
        [x1, x2, x3, x4]
    } else {
        unreachable!("blum")
    }
}