mod ballot;
use crate::ballot::AuditableBallot;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(AuditableBallot);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
