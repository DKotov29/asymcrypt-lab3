use std::ops::{Add, Div, Mul, Rem};
use malachite::{Integer, Natural, Rational};
use malachite::num::arithmetic::traits::{ExtendedGcd, JacobiSymbol, Mod, ModInverse, ModPow, Pow, Square};
use malachite::num::basic::traits::{One, Two, Zero};
use malachite::num::conversion::traits::FromStringBase;
use malachite::num::logic::traits::BitIterable;
use num_bigint::BigUint;
use num_integer::Integer as num_Integer;
use num_traits::{Num, Pow as num_pow};

pub fn format(m: &Natural, n: &Natural, r: &Natural) -> Option<Natural> {
    let l = n.bits().count().div_ceil(8usize);
    if m.bits().count().div_ceil(8usize) > l - 10 {
        return None;
    }
    let x = (Natural::from(255u16) * (Natural::from(2u8).pow(8 * (l as u64 - 2)))) + (m * (Natural::from(2u8).pow(64))) + r;

    Some(x)
    // Natural::from_string_base(16, format!("ff{}{m:x}{r:x}", "0".repeat(n.bits().count() - m.bits().count() - 20).as_str() ).as_str())
}

// pub fn generate(p: &Natural, q: &Natural) -> (Natural){
//     let n = p*q;
//     let oiler = (p - Natural::ONE) * (q - Natural::ONE);
//     let e = Natural::from(0b10000000000000001u64);
//     let d = e.mod_inverse(oiler).unwrap();
//     (d)
// }

pub fn encrypt(b: &Natural, x: &Natural, n: &Natural) -> (Natural, Natural, i8) { // x - message
    let y = x.mul(x + b).mod_op(n);

    let c1 = (x + b * (Natural::from(2u8).mod_inverse(n).unwrap())).mod_op(n).mod_op(Natural::from(2u8));
    let c2 = (x + b * (Natural::from(2u8).mod_inverse(n).unwrap())).jacobi_symbol(n);
    // let c2 = (x + b / (Natural::from(2u8))).jacobi_symbol(n);
    (y, c1, c2)
}


pub fn decrypt(b: &Natural, y: &Natural, p: &Natural, q: &Natural, c1: &Natural, n: &Natural, c2: i8) -> Integer {
    // let x = (-(&b / Natural::from(2u8)) + Integer::from(y + b.pow(2u64) / Natural::from(4u8))).square().mod_op(Integer::from(n));
    let x = blum(&(y + (b.pow(2u64) * (Natural::from(4u8).mod_inverse(n).unwrap()))), p, q);
    println!("loking for c1: {c1} c2: {c2}");

    let x = {
        let mut m = None;
        for xx in x {
            let nn = Integer::from(n);
            let xx = xx.mod_op(nn);
            let c11 = (&xx + Integer::from(b) * (Integer::from(&Natural::from(2u8).mod_inverse(n).unwrap())))
                .mod_op(Integer::from(n)).mod_op(Integer::from(2u8));
            let c22 = (&xx + Integer::from(b) * (Integer::from(Natural::from(2u8).mod_inverse(n).unwrap()))).jacobi_symbol(Integer::from(n));
            // let c22 = (&xx + Integer::from(b) / (Integer::from(Natural::from(2u8)))).jacobi_symbol(Integer::from(n));

            println!("c11,c22,x: {c11} {c22} {:x}", &xx);

            if c11 == Integer::from(c1) && c22 == c2 {
                m = Some(xx);
                println!("set some");
                // break;
            }
        }
        if m.is_none() {
            unreachable!("decrypt");
        } else {
            m.unwrap()
        }
    };
    (x - Integer::from(b) * Integer::from(Natural::TWO.mod_inverse(n).unwrap())).mod_op(Integer::from(n))
}

pub fn blum<'a>(y: &Natural, mut p: &'a Natural, mut q: &'a Natural) -> [Integer; 4] {
    // let s1 = y.mod_pow((p + Natural::ONE) / Natural::from(4u8), p);
    // let s2 = y.mod_pow((q + Natural::ONE) / Natural::from(4u8), q);

    let n = p * q;

    if p < q {
        std::mem::swap(&mut p, &mut q);
    }
    let (s1, s2);
    {
        let y = BigUint::from_str_radix(format!("{y}").as_str(), 10u32).unwrap();
        let p = BigUint::from_str_radix(format!("{p}").as_str(), 10u32).unwrap();
        let q = BigUint::from_str_radix(format!("{q}").as_str(), 10u32).unwrap();

        // let y1 = (y.clone()).pow(&p + &BigUint::from(1u8)).sqrt().sqrt();
        s1 = Natural::from_owned_limbs_asc((&y).modpow(
            // &BigUint::from(1u8),
            &((&p + &BigUint::from(1u8))/&BigUint::from(4u8)),
            &p,
        ).to_u64_digits());
        // let y2 = (y.clone()).pow(&q + &BigUint::from(1u8)).sqrt().sqrt();
        s2 = Natural::from_owned_limbs_asc((&y).modpow(
            &((&q + &BigUint::from(1u8))/&BigUint::from(4u8)),
            &q,
        ).to_u64_digits());
    }
    println!("s1: {s1}, s2: {s2}");
    let (gcd, u, v) = p.extended_gcd(q);
    // if p > q {
    //     std::mem::swap(&mut u, &mut v);
    // }
    if gcd == Natural::ONE {
        let x1 = ((&u * Integer::from(p) * Integer::from(&s2)) - (&v * Integer::from(q) * Integer::from(&s1))).mod_op(Integer::from(&n));
        let x2 = ((&u * Integer::from(p) * Integer::from(&s2)) + (&v * Integer::from(q) * Integer::from(&s1))).mod_op(Integer::from(&n));
        let x3 = Integer::from(&n) - &x1;
        let x4 = Integer::from(&n) - &x2;
        // let x3 = (-(&u * Integer::from(p) * Integer::from(&s2)) - (&v * Integer::from(q) * Integer::from(&s1))).mod_op(Integer::from(&n));
        // let x4 = (-(&u * Integer::from(p) * Integer::from(&s2)) + (&v * Integer::from(q) * Integer::from(&s1))).mod_op(Integer::from(&n));

        [x1, x2, x3, x4]
    } else {
        unreachable!("blum")
    }
}