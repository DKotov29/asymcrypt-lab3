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
use malachite::num::arithmetic::traits::{ExtendedGcd, FloorSqrt, Pow, Square};
use malachite::num::basic::traits::{One, Two};
use malachite::num::conversion::traits::FromStringBase;
use malachite::num::logic::traits::BitIterable;
use rand::{random, Rng};

fn main() {
    let b = Natural::TWO.pow(2u64);
    // let vec = par_generate(4, 2);
    let (p, q) = (
        // vec.get(0).unwrap().clone(),
        // vec.get(1).unwrap().clone()
        Natural::from_string_base(10u8, "56596653542729861995492937782359306906328022911586618389679946595578401780476997818817303845194306612334054692212450849184483786523525689986901572630509890049719171972406774742938379984251943256036320644841243114624857200210046651107855298261725816502012688096715592303666013021915301443948554705915447953847749758045150895092976132564502468609490384056680011895607158209190044371373807285132926418158351710027255178644742882315183009694217183591304493012361763548261928557394965818962355517805032436192820905814742728395942148296768460487126258243143110572580769612248659935275617418754618769369216444146408884759496419666995786175579").unwrap(),
        Natural::from_string_base(10u8, "63789964463407789677757613808706835625070646632683647164679070271771666214655985109797052357690224530972934384300057494951606365327174826564233935829828455444671111056075399457286933428493298037880224975240724353806727056370819492463089686092976392558836739252725798108983370932200718982172959059781889006684687012209128074703959671266934430079819544478342373677768910174694794303391088910671967313039294232277423719052498872212100211559016454017876022953683919328194372248251204232837260296118464618414597210519097738558902843100598422299268100844519953049892676692621941664475273372207993169386560774501518600038779296219199322483455").unwrap()
        );

    println!("p: {p}\nq: {q}");
    let n = &p*&q;
    let m = Natural::from(123456u64);
    let r = Natural::from(random::<u64>());
    let x = cryptosystem::format(&m, &n, &r ).unwrap();
    let (y, c1, c2) = cryptosystem::encrypt(&b, &x, &n);
    let (is_it_x) = cryptosystem::decrypt(&b,&y,&p,&q,&c1,&n, c2);
    println!("{x}\n{is_it_x}");


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
                {
                    if gen.lock().unwrap().len() >= amount {
                        break;
                    }
                }
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
