use std::{fs::File, io::Write};

use crate::pumpfun_struct::Root;

mod pumpfun_struct;

fn main() {
    if let Ok(pumpfun_json) = File::open("json_template.json") {
        let pumpfun_structure: Root = serde_json::from_reader(pumpfun_json).expect("Nevyparsoval lol");
        File::create("json.json").unwrap().write_all(serde_json::to_string(&pumpfun_structure).unwrap().as_bytes()).unwrap();
    }
}
