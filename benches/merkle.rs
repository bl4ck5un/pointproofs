#[macro_use]
extern crate bencher;
extern crate veccom;

use bencher::Bencher;
use veccom::merkle::paramgen::*;
use veccom::merkle::commit::*;
use veccom::merkle::verify::*;
use veccom::merkle::prove::*;

benchmark_group!(benches, bench_com, bench_prove, bench_verify, bench_commit_update, bench_proof_update);
benchmark_main!(benches);

fn bench_com(b: &mut Bencher) {
    let n = 1000usize;

    let params = paramgen(n);

    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is message number {}", i);
        values.push(s.into_bytes());
    }
    
    b.iter(|| { 
        commit(&params, &values)
    });
}

fn bench_prove(b: &mut Bencher) {
    let n = 1000usize;

    let params = paramgen(n);

    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is message number {}", i);
        values.push(s.into_bytes());
    }        
    let mut i : usize = 0;
    b.iter(|| {
        let p = prove(&params, &values, i);
        i = (i+1)%n;
        p
    });
}

fn bench_verify(b: &mut Bencher) {
    let n = 1000usize;

    let params =  paramgen(n);

    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is message number {}", i);
        values.push(s.into_bytes());
    }
    let com = commit(&params, &values);
    let mut proofs = Vec::with_capacity(n);
    for i in 0..n {
        proofs.push(prove(&params, &values, i));
    }

    let mut i : usize = 0;
    b.iter(|| {
        assert!(verify(&params, &com, &proofs[i], &values[i], i));
        i = (i+1)%n;
    });
}

fn bench_commit_update(b: &mut Bencher) {
    let n = 1000usize;

    let params = paramgen(n);

    let mut old_values = Vec::with_capacity(n);
    let mut new_values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is old message number {}", i);
        old_values.push(s.into_bytes());
        let t = format!("this is new message number {}", i);
        new_values.push(t.into_bytes());
    }
    let mut i : usize = 0;
    let mut proofs = Vec::with_capacity(n);
    for i in 0..n {
        proofs.push (prove(&params, &old_values, i));
    }

    b.iter(|| {
        commit_update(&params, i, &proofs[i], &new_values[i]);
        i = (i+1)%n;
    });
}

fn bench_proof_update(b: &mut Bencher) {
    let n = 1000usize;
    let update_index = n/2;  // We will update message number n/2 and then benchmark changing proofs for others


    let params = paramgen(n);

    let mut old_values = Vec::with_capacity(n);
    
    for i in 0..n {
        let s = format!("this is old message number {}", i);
        old_values.push(s.into_bytes());
    }

    let mut proofs = Vec::with_capacity(n);
    for i in 0..n {
        proofs.push(prove(&params, &old_values, i));
    }
    // Copy over the proof of the updated value in order to avoid mutable borrow isues in the proof_update
    let mut proof_of_updated_value = Vec::new();
    for i in 0..proofs[update_index].len() {
        proof_of_updated_value.push(proofs[update_index][i]);
    }

    let new_value = format!("this is new message number {}", update_index).into_bytes();
    
    let mut i : usize = 0;
    b.iter(|| {
        proof_update(&params, &mut proofs[i], i, update_index, &proof_of_updated_value, &new_value);
        i = (i+1)%n;
        if i==update_index { // skip update_index
            i = (i+1)%n;
        }
        proofs[i].len();
    });
}
