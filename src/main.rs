use clap::Parser;
use serde_json::Value;

/// Export Filecoin data from a remote Lotus node
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Epoch to start the export from
    #[clap(short, long, value_parser, default_value_t = 0)]
    from: u32,

    /// Epoch to end the export
    #[clap(short, long, value_parser, default_value_t = 0)]
    to: u32,
}

/// Get the latest chain head
fn get_chain_head() -> Result<u32, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let data = r#"
        {
            "jsonrpc": "2.0",
            "method": "Filecoin.ChainHead",
            "id": 1,
            "params": []
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    let res = client.post("https://api.node.glif.io").json(&v).send();

    let json = res.unwrap().json::<Value>().unwrap();
    Ok(json["result"]["Height"].as_u64().unwrap() as u32)
}

fn main() {
    let args = Args::parse();

    // If both from and to are 0, start from the current chain head
    if args.from == 0 && args.to == 0 {
        println!("Following the chain from the current head");
        println!("The chain head is: {}", get_chain_head().unwrap());
    } else {
        println!("Exporting from {} to {}", args.from, args.to)
    }
}
