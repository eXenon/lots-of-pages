use std::collections::HashMap;
use strfmt::strfmt;

const CONTENTS: &'static str = include_str!("file.html");

fn gen_name() -> String {
    return "alex".to_string();
}

pub fn gen() -> String {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), gen_name());
    strfmt(&CONTENTS, &vars).unwrap()
}
