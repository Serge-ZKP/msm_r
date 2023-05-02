pub mod fields;
pub mod curves;
pub mod panther12;

#[macro_use]
extern crate derivative;

fn main() {
    use crate::curves::g1::{G1Projective as G, G1Affine as GAffine};
    use ark_ff::UniformRand;
    use std::time::Instant;
    use crate::fields::fr::Fr as ScalarField;
    use ark_ec::VariableBaseMSM;
    let mut rng = ark_std::test_rng();


    // Let's sample uniformly random group elements:
    //const DEG: usize = 1000000;
    const DEG: usize = 10000;
    let mut g = vec![];
    let mut s = vec![];
    for _ in 0..DEG {
        g.push(GAffine::rand(&mut rng));
        s.push(ScalarField::rand(&mut rng));
    }

    let now = Instant::now();
    //let _r = G::msm(g.as_slice(), s.as_slice()).unwrap();
    let _r = G::msm(&g, &s).unwrap();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
