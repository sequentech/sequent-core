use crate::ballot::*;
use ed25519_dalek::{Digest, Sha512};
use num_bigint::BigUint;
use num_traits::Num;
use strand::backend::numb::{BigintCtx, P2048};
use strand::context::Ctx;
use strand::elgamal::*;

fn recreate_encrypt_answer(public_key: &Pk, choice: &ReplicationChoice) -> CyphertextChoice {
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

pub fn recreate_encrypt_cyphertext(ballot: &Ballot) -> Cyphertext {
    assert_eq!(
        ballot.replication.choices.len(),
        ballot.config.payload.pks.len(),
    );
    assert_eq!(
        ballot.config.payload.pks.len(),
        ballot.config.payload.configuration.questions.len(),
    );
    let mut choices = vec![];
    for i in 0..ballot.replication.choices.len() {
        let cyphertext_answer = recreate_encrypt_answer(
            &ballot.config.payload.pks[i],
            &ballot.replication.choices[i],
        );
        choices.push(cyphertext_answer);
    }
    Cyphertext {
        issue_date: ballot.replication.issue_date.clone(),
        choices,
    }
}

pub fn hash_to(ballot: &Ballot) -> String {
    let cyphertext = recreate_encrypt_cyphertext(&ballot);
    let ballot_str = serde_json::to_string(&cyphertext).unwrap();
    let mut hasher = Sha512::new();
    hasher.update(ballot_str.as_bytes());
    let hashed = hasher.finalize();
    hex::encode(&hashed)
}

#[cfg(test)]
mod tests {
    use crate::ballot::*;
    use crate::encrypt::*;
    use std::fs;

    #[test]
    fn parse_ballot() {
        let contents = fs::read_to_string("fixtures/ballot.json")
            .expect("Something went wrong reading the file");
        let ballot: Ballot = serde_json::from_str(&contents).unwrap();
        let sha512_ballot = hash_to(&ballot);
        assert_eq!(&sha512_ballot, &ballot.ballot_hash);
        assert_eq!(
            &sha512_ballot,
            "e182b0ee3654d252fec6b47b210e8e2a5ce051b6702ae1da6516dab6a47f6df507b0ce6dfd0d9b267c9e5ede8be8732b365365f4e8cb39159d75d7f4a9b80cc2"
        )
    }
}
