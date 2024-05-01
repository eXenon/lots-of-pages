use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::{collections::HashMap, ops::Index};
use strfmt::strfmt;

const CONTENTS: &'static str = include_str!("file.html");
const FIRST: [&'static str; 64] = [
    "Abigail",
    "Alexander",
    "Alice",
    "Andrew",
    "Ava",
    "Aster",
    "Benjamin",
    "Brianna",
    "Ben",
    "Brian",
    "Charlotte",
    "Chloe",
    "Christopher",
    "Daniel",
    "David",
    "Diana",
    "Ella",
    "Emily",
    "Eric",
    "Ethan",
    "Faith",
    "Finn",
    "Gabriel",
    "George",
    "Grace",
    "Hailey",
    "Hannah",
    "Henry",
    "Holly",
    "Ian",
    "Isabella",
    "Jack",
    "Jacob",
    "Jessica",
    "Katherine",
    "Kevin",
    "Lauren",
    "Liam",
    "Lily",
    "Matthew",
    "Mia",
    "Michael",
    "Natalie",
    "Noah",
    "Olivia",
    "Owen",
    "Patrick",
    "Penelope",
    "Quinn",
    "Rachel",
    "Ryan",
    "Samuel",
    "Sarah",
    "Sophia",
    "Thomas",
    "Tyler",
    "Vanessa",
    "Victoria",
    "William",
    "Wyatt",
    "Xavier",
    "Yvonne",
    "Zachary",
    "Zoe",
];
const MIDDLE: [&'static str; 128] = [
    "Aaron",
    "Abigail",
    "Addison",
    "Adrian",
    "Aiden",
    "Alexander",
    "Amelia",
    "Andrew",
    "Anna",
    "Anthony",
    "Aria",
    "Asher",
    "Aubrey",
    "Audrey",
    "Ava",
    "Averon",
    "Avery",
    "Bella",
    "Benjamin",
    "Brooklyn",
    "Caleb",
    "Cameron",
    "Caroline",
    "Carter",
    "Charles",
    "Charlotte",
    "Chloe",
    "Christian",
    "Christopher",
    "Claire",
    "Colton",
    "Connor",
    "David",
    "Dylan",
    "Eleanor",
    "Eli",
    "Elijah",
    "Elizabeth",
    "Ella",
    "Ellie",
    "Elois",
    "Emily",
    "Emma",
    "Ethan",
    "Evan",
    "Evelyn",
    "Everly",
    "Ezra",
    "Gabriel",
    "Genesis",
    "Grace",
    "Grace",
    "Grayson",
    "Hannah",
    "Harper",
    "Hazel",
    "Henry",
    "Hudson",
    "Hunter",
    "Isaac",
    "Isabella",
    "Isaiah",
    "Jack",
    "Jackson",
    "James",
    "Jasmine",
    "Jaxon",
    "Jayden",
    "John",
    "Jonathan",
    "Joseph",
    "Joshua",
    "Josiah",
    "Julia",
    "Julian",
    "Landon",
    "Layla",
    "Leah",
    "Leo",
    "Levi",
    "Liam",
    "Lily",
    "Lincoln",
    "Logan",
    "Lucas",
    "Luke",
    "Luna",
    "Lydia",
    "Madelyn",
    "Mason",
    "Mateo",
    "Matthew",
    "Mia",
    "Michael",
    "Mila",
    "Morgan",
    "Naomi",
    "Naru",
    "Natalie",
    "Nathan",
    "Noah",
    "Nolan",
    "Nora",
    "Oliver",
    "Olivia",
    "Owen",
    "Paisley",
    "Penelope",
    "Peyton",
    "Riley",
    "Ryan",
    "Sadie",
    "Savannah",
    "Scarlett",
    "Sebastian",
    "Skylar",
    "Sofia",
    "Sophia",
    "Sophie",
    "Stella",
    "Theodore",
    "Thomas",
    "Victoria",
    "Violet",
    "William",
    "Wyatt",
    "Zenya",
    "Zoey",
];
const LAST: [&'static str; 64] = [
    "Anderson", "Baker", "Carter", "Davis", "Edwards", "Fisher", "Gibson", "Harris", "Jackson",
    "Bailey", "Russell", "Powell", "Long", "Butler", "Cole", "Ross", "Foster", "Simmons", "Perry",
    "Barnes", "Howard", "Wood", "Bryant", "Gonzalez", "Martinez", "Lopez", "Kelly", "Lewis",
    "Mitchell", "Nelson", "Owens", "Parker", "Reynolds", "Smith", "Taylor", "Lee", "Allen", "Cook",
    "Hill", "Moore", "Rogers", "Murphy", "Bell", "Ward", "Stewart", "Morgan", "Roberts", "Scott",
    "Williams", "Johnson", "Thomas", "Robinson", "White", "Hall", "Walker", "Young", "Adams",
    "Brown", "Clark", "Evans", "Green", "Hughes", "Jenkins", "King",
];

const LETTERS: [&'static str; 16] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
];

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

struct Parameters {
    first: String,
    middle: String,
    last: String,
    letter: String,
}

fn gen_params(rng: &mut StdRng) -> Parameters {
    let id1 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let id2 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let id3 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let id4 = usize::try_from(rng.next_u64()).unwrap_or(0);
    Parameters {
        first: FIRST[id1 % FIRST.len()].into(),
        middle: MIDDLE[id2 % MIDDLE.len()].into(),
        last: LAST[id3 % LAST.len()].into(),
        letter: LETTERS[id4 % LETTERS.len()].into(),
    }
}

fn host_to_seed(host: &str) -> u64 {
    let mut parts: Vec<&str> = host.split('-').collect();
    let first = capitalize(parts.pop().unwrap_or(FIRST[0]));
    let middle = capitalize(parts.pop().unwrap_or(MIDDLE[0]));
    let last = capitalize(parts.pop().unwrap_or(LAST[0]));
    let first_i = FIRST
        .iter()
        .position(|&t| t == first)
        .and_then(|i| u64::try_from(i).ok())
        .unwrap_or(0);
    let middle_i = MIDDLE
        .iter()
        .position(|&t| t == middle)
        .and_then(|i| u64::try_from(i).ok())
        .unwrap_or(0);
    let last_i = LAST
        .iter()
        .position(|&t| t == last)
        .and_then(|i| u64::try_from(i).ok())
        .unwrap_or(0);
    return first_i + (middle_i >> 6) + (last_i >> 13);
}

pub fn gen(host: &str) -> String {
    let seed = host_to_seed(host);
    println!("Seed: {}", seed);
    let mut vars = HashMap::new();
    let mut rng = StdRng::seed_from_u64(seed);
    let params = gen_params(&mut rng);
    vars.insert("first".to_string(), params.first);
    vars.insert("middle".to_string(), params.middle);
    vars.insert("last".to_string(), params.last);
    vars.insert("letter".to_string(), params.letter);
    strfmt(&CONTENTS, &vars).unwrap()
}
