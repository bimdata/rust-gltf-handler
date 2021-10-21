use std::env;
use std::fs;
use std::io::prelude::*;
use std::str;
use base64;
use serde_json::Value;
use serde_json::json;

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read input json file
    let json = fs::read_to_string(filename)
        .expect("Error while reading JSON file.");

    // Deserialize json value
    let json: Value = serde_json::from_str(&json)
        .expect("Error while parsing JSON string.");

    // Get message field from json
    let message: String = json["message"].to_string();

    // Decode message
    let l = message.len();
    let message: Vec<u8> = base64::decode(&message[1..l-1])
        .expect("Error while decoding message.");
    let message: &str = str::from_utf8(&message)
        .expect("Error while converting message.");

    // Create output content
    let content: Value = json!({ "message": message });
    let content: String = content.to_string();

    // Write content to output json file
    let output = "output.json";
    let mut output = fs::File::create(output)
        .expect("Error while creating output file.");
    write!(output, "{}", content);
}
