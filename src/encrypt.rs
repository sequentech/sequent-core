use crate::ballot::*;
use ed25519_dalek::{Digest, Sha512};
use num_bigint::BigUint;
use num_traits::Num;
use strand::backend::num_bigint::{BigintCtx, P2048};
use strand::context::Ctx;
use strand::elgamal::*;

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum BallotError {
        ParseBigUint(uint_str: String, message: String) {}
        CryptographicCheck(message: String) {}
        ConsistencyCheck(message: String) {}
        Serialization(message: String) {}
    }
}

fn recreate_encrypt_answer(
    public_key: &Pk,
    choice: &ReplicationChoice,
) -> Result<CyphertextChoice, BallotError> {
    // create P2048 context
    let ctx = BigintCtx::<P2048>::new();

    // create public key
    let pk_bigint = BigUint::from_str_radix(&public_key.public_key, 10).map_err(|_| {
        BallotError::ParseBigUint(
            public_key.public_key.clone(),
            String::from("Error parsing public key"),
        )
    })?;
    let pk_bigint_e = ctx
        .element_from_bytes(&pk_bigint.to_bytes_le())
        .map_err(|_| {
            BallotError::CryptographicCheck(String::from("Error parsing public key as an element"))
        })?;
    let pk = PublicKey::from_element(&pk_bigint_e, &ctx);

    // parse plaintext
    let plaintext = BigUint::from_str_radix(&choice.plaintext, 10).map_err(|_| {
        BallotError::ParseBigUint(
            choice.plaintext.clone(),
            String::from("Error parsing plaintext"),
        )
    })?;
    let plaintext_e = ctx
        .element_from_bytes(&plaintext.to_bytes_le())
        .map_err(|_| {
            BallotError::CryptographicCheck(String::from("Error parsing plaintext as an element"))
        })?;

    // parse randomness
    let randomness = BigUint::from_str_radix(&choice.randomness, 10).map_err(|_| {
        BallotError::ParseBigUint(
            choice.randomness.clone(),
            String::from("Error parsing randomness"),
        )
    })?;
    let randomness_e = ctx.exp_from_bytes(&randomness.to_bytes_le()).map_err(|_| {
        BallotError::CryptographicCheck(String::from("Error parsing randomness as an element"))
    })?;

    if KeyType::P2048 != public_key.key_type {
        return Err(BallotError::ConsistencyCheck(String::from(
            "Invalid key type",
        )));
    }

    // encrypt / create cyphertext
    let cyphertext = pk.encrypt_with_randomness(&plaintext_e, &randomness_e);

    // convert to output format
    Ok(CyphertextChoice {
        gr: cyphertext.gr().0.to_str_radix(10),
        mhr: cyphertext.mhr().0.to_str_radix(10),
    })
}

pub fn recreate_encrypt_cyphertext(ballot: &AuditableBallot) -> Result<Cyphertext, BallotError> {
    // check ballot version
    // sanity checks for number of answers/choices
    if ballot.replication.choices.len() != ballot.config.payload.pks.len() {
        return Err(BallotError::ConsistencyCheck(String::from(
            "Number of public keys should match number of answers in the ballot",
        )));
    }
    if ballot.config.payload.pks.len() != ballot.config.payload.configuration.questions.len() {
        return Err(BallotError::ConsistencyCheck(String::from(
            "Number of public keys should match number of election questions",
        )));
    }
    let mut choices = vec![];
    for i in 0..ballot.replication.choices.len() {
        let cyphertext_answer = recreate_encrypt_answer(
            &ballot.config.payload.pks[i],
            &ballot.replication.choices[i],
        )?;
        choices.push(cyphertext_answer);
    }
    Ok(Cyphertext {
        issue_date: ballot.replication.issue_date.clone(),
        choices,
    })
}

pub fn hash_to(ballot: &AuditableBallot) -> Result<String, BallotError> {
    let cyphertext = recreate_encrypt_cyphertext(&ballot)?;
    let ballot_str = serde_json::to_string(&cyphertext)
        .map_err(|_| BallotError::Serialization(String::from("Error serializing cyphertext")))?;
    let mut hasher = Sha512::new();
    hasher.update(ballot_str.as_bytes());
    let hashed = hasher.finalize();
    Ok(hex::encode(&hashed))
}

#[cfg(test)]
mod tests {
    use crate::ballot::*;
    use crate::encrypt::*;
    use std::fs;

    fn read_ballot_fixture() -> AuditableBallot {
        let contents = fs::read_to_string("fixtures/ballot.json")
            .expect("Something went wrong reading the file");
        serde_json::from_str(&contents).unwrap()
    }

    #[test]
    fn test_parse_ballot() {
        let ballot = read_ballot_fixture();
        let sha512_ballot = hash_to(&ballot).unwrap();
        assert_eq!(&sha512_ballot, &ballot.ballot_hash);
        assert_eq!(
            &sha512_ballot,
            "bc15bf91def8033b8b586e929335c40e23ffc576a1bcb469909646222abcf6858e290b52f836cbb9744462c6869788878d88b22c8b4d9efd7cb750b700dba3e8"
        );
        let recreated_cyphertext = recreate_encrypt_cyphertext(&ballot).unwrap();
        assert_eq!(recreated_cyphertext, ballot.cyphertext);
    }

    #[test]
    fn test_recreate_encrypt_answer() {
        let ballot = read_ballot_fixture();
        let pk = Pk {
            key_type: KeyType::P2048,
            public_key: "invalid_key".to_string(),
        };
        let call_result = recreate_encrypt_answer(&pk, &ballot.replication.choices[0]);
        assert_eq!(
            call_result,
            Err(BallotError::ParseBigUint(
                pk.public_key.clone(),
                String::from("Error parsing public key"),
            ))
        );
    }
}
