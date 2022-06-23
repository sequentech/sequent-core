use crate::ballot::*;
use strand::backend::numb::{BigintCtx, P2048};
use num_bigint::BigUint;
use num_traits::Num;
use strand::elgamal::*;
use strand::context::Ctx;

fn recreate_encrypt_answer(
    public_key: &Pk,
    choice: &ReplicationChoice,
) -> CyphertextChoice {
    /*
    let p = BigUint::from_str_radix(&public_key.p, 10).unwrap();
    let q = BigUint::from_str_radix(&public_key.q, 10).unwrap();
    let g = BigUint::from_str_radix(&public_key.g, 10).unwrap();
    let co_factor = BigUint::from_str_radix(&public_key.y, 10).unwrap();

    // assert!(g.legendre(&p) == 1);

    let params = P2048 {
        generator: g,
        modulus: p,
        exp_modulus: q,
        co_factor,
    };
    let ctx = BigintCtx {
        params: params,
    };*/
    let ctx = BigintCtx::<P2048>::new();
    let sk = PrivateKey::gen(&ctx);
    let pk = sk.get_public();
    let plaintext: BigUint = BigUint::from_str_radix(&choice.plaintext, 10).unwrap();
    // let plaintext_enc: BigUint = ctx.encode(&plaintext).unwrap(); // REQUIRED¿¿??
    let randomness: BigUint = BigUint::from_str_radix(&choice.randomness, 10).unwrap();
    let cyphertext = pk.encrypt_ext(&plaintext, &randomness);
    CyphertextChoice {
        alpha: cyphertext.mhr().to_str_radix(10),
        beta: cyphertext.gr().to_str_radix(10),
    }
}