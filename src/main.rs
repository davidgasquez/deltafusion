use clap::Parser;
use serde_json::Value;

/// Export Filecoin data from a remote Lotus node
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Entity to export
   #[clap(short, long, value_parser)]
   entity: String,

   /// Epoch to start the export from
   #[clap(short, long, value_parser, default_value_t = 2100000)]
   from: u32,

   /// Epoch to end the export
   #[clap(short, long, value_parser, default_value_t = 2200000)]
   to: u32,
}

fn main() {
    let args = Args::parse();

    println!("Exporting {} from {} to {}", args.entity, args.from, args.to);

    let data = r#"
        {
            "jsonrpc": "2.0",
            "method": "Filecoin.ChainHead",
            "id": 1,
            "params": []
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://api.node.glif.io")
        .json(&v)
        .send();

    let json = res.unwrap().json::<Value>().unwrap();
    println!("The latest height is: {}", json["result"]["Height"]);
}
