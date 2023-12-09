#![feature(int_roundings)]

mod cryptosystem;
mod prime_test;
mod rand_generator;

use std::ops::Rem;

extern crate core;

use std::str::FromStr;
use std::sync::Mutex;
use rayon::prelude::*;
use malachite::Natural;
use malachite::num::arithmetic::traits::{ExtendedGcd, FloorSqrt, Mod, ModPow, Pow, Square};
use malachite::num::basic::traits::{One, Two};
use malachite::num::conversion::traits::FromStringBase;
use malachite::num::logic::traits::BitIterable;
use num_bigint::BigUint;
use rand::{random, Rng};

fn main() {

    let b = Natural::TWO.pow(2u64);
    println!("b: {b:x}");
    // let vec = par_generate(2, 2);
    let (p, q) = (
        // vec.get(0).unwrap().clone(),
        // vec.get(1).unwrap().clone()
        Natural::from_string_base(16u8, "f469f1cc2492eeb6c4c845b0253081738e0a4518777caaf115c4eb8e8229957452f795eafe92acffa850859b30e468f81e2a3b8c425a4edda06b3fd59b88354900046dd15284c77f2b0f55d6873632b74f833d23305977e812c197a6df77113cf6b9ad15d1276badf53045b5acf1933b3767ff9e2e640c415ba3e5d38b059b95687a0264975317c2d7a8f5054958ff224298d71361f0123ee0c8a792314652933ccda9afaaed9c9090e1568dcfa91c8f791d0d7bfec8662c338bead6ce0d7804c0771f251905f1752bfb7e5e550d6566adf6ef7cc88b1653a75e87cf9a37d557995e3d180bfbf418c4cb34d4cfe28713e7c31fbc4f589665104d5cfa4ef70622878ff76b1257ea1f").unwrap(),
        Natural::from_string_base(16u8, "74b8eb2a9c215c9bb6522fdb0c7e802b356fd6b5e563752925b0272596dc31adeceec639a06e49f4e7866cfd2382ecb263eeaeb00391bca77d3ff59382df82db7532ce55b111de6755dc1ca323619e057e0ef69060844f7dc081d41f19000d148723727cf762764502219adb06e41f05d69293b75f6f3f2b32de1debf69c9f06082f3a0e36537e604c1e4cc47cc276afd95574b12b3e99c20eed59cd32192d87be0fa028e07bdde49a0b69c0086e5f05bbafeb928d7377b7061c09b2a1676d996a6ab87c8c7fd2211adc9aae241d3b12959406c3766d945a20ef8fa385b17bec9e34846e92b8f228a1cae378b3f3af663aa0c5a5d8707766c5a823a7c57cb09f6749f872b40eeee7").unwrap()
    );

    println!("p: {p:x}\nq: {q:x}");
    println!("n: {:x}", &p * &q);
    let n = &p * &q;
    let m = Natural::from(123456u64);
    println!("m: {m:x}");
    let r = Natural::from(random::<u64>());
    let x = cryptosystem::format(&m, &n, &r).unwrap();
    // let x = m;
    println!("x: {x:x}");
    //;
    let (y, c1, c2) = cryptosystem::encrypt(&b, &x, &n);
    println!("y (encrypted x): {y:x}");
    let (is_it_x) = cryptosystem::decrypt(&b, &y, &p, &q, &c1, &n, c2);
    println!("x: {x}\ndecrypted: {is_it_x}");

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
                let gen_len: usize = 33; // in u64, 32 * 64 = 2048 bits
                let vec = rand_generator::generate(gen_len);
                let num = Natural::from_owned_limbs_asc(vec);
                if (&num - Natural::from(3u8)).rem(Natural::from(4u8)) != 0 {
                    continue;
                }
                // {
                //     if gen.lock().unwrap().len() >= amount {
                //         break;
                //     }
                // }
                // let mn = num.bits().count();
                // if mn < 2048 {
                //     continue;
                // }
                found = prime_test::test(&num);
                number = num;
            }
            println!("{number}");
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
