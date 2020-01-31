use num_bigint::BigUint;

fn main() {
    optimal();
}

fn optimal() {
    let mut primes = Vec::new();
    primes.push(BigUint::from(2u64));
    println!("{}", 2);
    let mut i = BigUint::from(1u64);
    'next: loop {
        *(&mut i) += BigUint::from(2u64);
        for p in &primes {
            if &i * &i < *p {
                break;
            } else if &i % p == BigUint::from(0u64) {
                continue 'next;
            }
        }
        println!("{}", &i);
        primes.push(i.clone());
    }
}

// fn unoptimal() {
//     let mut i = BigUint::from(2u64);
//     'next: loop {
//         *(&mut i) += BigUint::from(1u64);
//         let mut j = BigUint::from(1u64);
//         while j < (&i - BigUint::from(1u64)) {
//             *(&mut j) += BigUint::from(1u64);
//             if &i % &j == BigUint::from(0u64) {
//                 continue 'next;
//             }
//         }
//         println!("unoptimal {}", &i);
//     }
// }