use std::{collections::HashMap, intrinsics::mir::Len};
use strfmt::strfmt;

const CONTENTS: &'static str = include_str!("file.html");
const FIRST: [&'static str; 32] = [
    "Alice",
    "Benjamin",
    "Charlotte",
    "Daniel",
    "Emily",
    "Finn",
    "Grace",
    "Henry",
    "Isabella",
    "Jack",
    "Katherine",
    "Liam",
    "Mia",
    "Noah",
    "Olivia",
    "Patrick",
    "Quinn",
    "Rachel",
    "Samuel",
    "Sophia",
    "Thomas",
    "Victoria",
    "William",
    "Xavier",
    "Yvonne",
    "Zachary",
    "Abigail",
    "Ethan",
    "Hannah",
    "Jacob",
    "Lily",
    "Matthew",
];
const LAST: [&'static str; 32] = [
    "Anderson", "Baker", "Carter", "Davis", "Edwards", "Fisher", "Gibson", "Harris", "Jackson",
    "Kelly", "Lewis", "Mitchell", "Nelson", "Owens", "Parker", "Reynolds", "Smith", "Taylor",
    "Walker", "Young", "Adams", "Brown", "Clark", "Evans", "Green", "Hughes", "Jenkins", "King",
    "Morgan", "Roberts", "Scott", "Williams",
];

fn gen_name(rng: StdRng) -> String {
    return FIRST[rng.next_u32() % Len(FIRST)];
}

pub fn gen(seed: u64) -> String {
    let mut vars = HashMap::new();
    let rng = StdRng::seed_from_u64(seed);
    let rng = vars.insert("name".to_string(), gen_name());
    strfmt(&CONTENTS, &vars).unwrap()
}
