use crate::ballot::*;
use num_bigint::BigUint;
use num_traits::Num;
use strand::backend::numb::{BigintCtx, P2048};
use strand::context::Ctx;
use strand::elgamal::*;

pub fn recreate_encrypt_answer(public_key: &Pk, choice: &ReplicationChoice) -> CyphertextChoice {
    // create P2048 context
    let ctx = BigintCtx::<P2048>::new();
    // create public key
    let pk_bigint = BigUint::from_str_radix(&public_key.public_key, 10).unwrap();
    let pk = PublicKey::from_element(&pk_bigint, &ctx);
    // parse plaintext
    let plaintext = BigUint::from_str_radix(&choice.plaintext, 10).unwrap();

    // parse randomness
    let randomness = BigUint::from_str_radix(&choice.randomness, 10).unwrap();

    // valid checks
    assert!(ctx.is_valid_element(&plaintext));
    assert_eq!(public_key.key_type, KeyType::P2048);

    // encrypt / create cyphertext
    let cyphertext = pk.encrypt_ext(&plaintext, &randomness);

    // convert to output format
    CyphertextChoice {
        gr: cyphertext.gr().to_str_radix(10),
        mhr: cyphertext.mhr().to_str_radix(10),
    }
}
