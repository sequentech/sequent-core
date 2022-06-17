mod ballot;
use crate::ballot::Ballot;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(Ballot);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
