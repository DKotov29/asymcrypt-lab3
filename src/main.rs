#![feature(int_roundings)]
mod cryptosystem;
mod prime_test;
mod rand_generator;

use std::ops::Rem;

extern crate core;

use std::str::FromStr;
use std::sync::Mutex;
use std::time::Instant;
use rayon::prelude::*;
use malachite::Natural;
use malachite::num::arithmetic::traits::{FloorSqrt, Square};
use malachite::num::basic::traits::One;
use malachite::num::logic::traits::BitIterable;

fn main() {
    let vec = par_generate(4, 2);
    let (p, q) = (
        vec.get(0).unwrap().clone(),
        vec.get(1).unwrap().clone());
    let n = &p*&q;
    // println!("{:?}", cryptosystem::format(&Natural::from(0b111111111u), &Natural::ONE, &Natural::from_owned_limbs_asc(rand_generator::generate(1))));
}
fn par_generate(at_once: usize, amount: usize) -> Vec<Natural> {
    let mut vec = Vec::with_capacity(amount);
    let mut gen = Mutex::new(Vec::with_capacity(amount));
    while gen.lock().unwrap().len() < amount {
        (0..at_once).par_bridge().for_each(|_| {
            let mut found = false;
            let mut number = Natural::ONE;
            while !found {
                let gen_len: usize = 256 / 64; // in u64
                let vec = rand_generator::generate(gen_len);
                let num = Natural::from_owned_limbs_asc(vec);
                if (&num - Natural::from(3u8)).rem(Natural::from(4u8)) != 0 {
                    continue;
                }
                let mn = num.bits().count();
                if mn < 256 {
                    continue;
                }
                found = prime_test::test(&num);
                number = num;
            }
            gen.lock().unwrap().push(number);
            if gen.lock().unwrap().len() >= amount {
                rayon::yield_now();
            }
        });
    }
    for i in &gen.lock().unwrap()[0..amount] {
        vec.push(i.clone());
    }
    vec
}
