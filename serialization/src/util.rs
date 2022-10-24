use std::{error::Error, fs::File, io::BufReader, path::Path};

use crate::model::{self, AdModel};

pub fn to_json() -> Option<String> {
    let am = model::AdModel {
        id: 1,
        title: "总经理金晶科技大".to_owned(),
        creatives: vec![1, 2, 3, 4, 5],
        bid: 98989000000,
        cpa_bid: 198989000000,
        landing_url: "https://www.google.com".to_owned(),
        bid_type: model::BidType::CPA,
    };

    match serde_json::to_string(&am) {
        Ok(str) => {
            println!("{}", str);
            Some(str.to_string())
        }
        Err(err) => {
            println!("err {:?}", err);
            None
        }
    };
    None
}

pub fn to_obj() -> Option<model::AdModel> {
    let json_str = "{\"id\":1024,\"title\":\"总经理金晶科技大\",\"creatives\":[1,2,3,4,5],\"bid\":12345000000,\"cpa_bid\":398989000000,\"landing_url\":\"https://www.google.com\",\"bid_type\":\"cpa\"}";
    let an: AdModel = serde_json::from_str(json_str).unwrap();
    println!("an {:?}", an);
    Some(an)
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<model::AdModel, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
