use crate::ballot::*;
use num_bigint::BigUint;
use num_traits::Num;
use strand::backend::numb::{BigintCtx, P2048};
use strand::context::Ctx;
use strand::elgamal::*;

pub fn recreate_encrypt_answer(public_key: &Pk, choice: &ReplicationChoice) -> CyphertextChoice {
    let y = BigUint::from_str_radix(&public_key.y, 10).unwrap();
    let ctx = BigintCtx::<P2048>::new();
    let pk = PublicKey::from_element(&y, &ctx);
    let plaintext = BigUint::from_str_radix(&choice.plaintext, 10).unwrap();
    // let plaintext_enc: BigUint = ctx.encode(&plaintext).unwrap(); // REQUIRED¿¿??
    assert!(ctx.is_valid_element(&plaintext));
    let randomness = BigUint::from_str_radix(&choice.randomness, 10).unwrap();
    let cyphertext = pk.encrypt_ext(&plaintext, &randomness);
    CyphertextChoice {
        alpha: cyphertext.gr().to_str_radix(10),
        beta: cyphertext.mhr().to_str_radix(10),
    }
}
