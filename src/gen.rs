use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::collections::HashMap;
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

const FIELDS: [&'static str; 16] = [
    "Astrobiogenetics",
    "Bioaeroacoustics",
    "Biomechatronics",
    "Chronoastrochemistry",
    "Cryoastrobiology",
    "Cryogenomics",
    "Cyberneticontology",
    "Exoplanetology",
    "Geoneurogenetics",
    "Geopaleobiotechnology",
    "Nanobiophysics",
    "Paleoecopharmacology",
    "Psychoneuroimmunogenetics",
    "Quantumomics",
    "Xenobiogeology",
    "Xenogenomics",
];

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

struct Person {
    first: String,
    middle: String,
    last: String,
    letter: String,
    field: String,
}

fn gen_person(rng: &mut StdRng) -> Person {
    let id1 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let id2 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let id3 = usize::try_from(rng.next_u64()).unwrap_or(0);
    let first = FIRST[id1 % FIRST.len()].into();
    let middle = MIDDLE[id2 % MIDDLE.len()].into();
    let last = LAST[id3 % LAST.len()].into();
    return person_id_to_person(first, middle, last);
}

fn parse_host(host: String) -> Person {
    let subdomain = host.split('.').into_iter().next().unwrap_or("");
    let mut parts: Vec<&str> = subdomain.split('-').collect();

    // Last name
    let last_part = capitalize(parts.pop().unwrap_or(LAST[0]));
    let last = if LAST.iter().position(|x| *x == last_part) == None {
        LAST[0].to_string()
    } else {
        last_part
    };

    // Middle name
    let middle_part = capitalize(parts.pop().unwrap_or(MIDDLE[0]));
    let middle = if MIDDLE.iter().position(|x| *x == middle_part) == None {
        MIDDLE[0].to_string()
    } else {
        middle_part
    };

    // First name
    let first_part = capitalize(parts.pop().unwrap_or(FIRST[0]));
    let first = if FIRST.iter().position(|x| *x == first_part) == None {
        FIRST[0].to_string()
    } else {
        first_part
    };

    return person_id_to_person(first, middle, last);
}

fn person_id_to_seed(first: &String, middle: &String, last: &String) -> u64 {
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

fn person_id_to_letter_field(first: &String, middle: &String, last: &String) -> [String; 2] {
    let seed: usize = usize::try_from(person_id_to_seed(&first, &middle, &last)).unwrap_or(0);
    let field = FIELDS[seed % FIELDS.len()].into();
    let letter = LETTERS[seed % LETTERS.len()].into();
    return [field, letter];
}

fn person_id_to_person(first: String, middle: String, last: String) -> Person {
    let [field, letter] = person_id_to_letter_field(&first, &middle, &last);
    return Person {
        first,
        middle,
        last,
        letter,
        field,
    };
}

fn person_to_seed(person: &Person) -> u64 {
    return person_id_to_seed(&person.first, &person.middle, &person.last);
}

fn insert_person(vars: &mut HashMap<String, String>, person: Person, suffix: &String) {
    vars.insert("first".to_string() + suffix, person.first);
    vars.insert("middle".to_string() + suffix, person.middle);
    vars.insert("last".to_string() + suffix, person.last);
    vars.insert("letter".to_string() + suffix, person.letter);
    vars.insert("field".to_string() + suffix, person.field);
}

pub fn gen(host: &str) -> String {
    let is_root = host == "";
    let current = parse_host(host.to_string());
    let seed = person_to_seed(&current);
    let mut vars = HashMap::new();
    let mut rng = StdRng::seed_from_u64(seed);

    insert_person(&mut vars, current, &"_current".to_string());
    let title = if is_root {
        "Welcome!".to_string()
    } else {
        strfmt("{first_current} {middle_current} {last_current}", &vars)
            .unwrap_or("Welcome!".to_string())
    };
    vars.insert("title".to_string(), title);
    insert_person(&mut vars, gen_person(&mut rng), &"_0".to_string());
    insert_person(&mut vars, gen_person(&mut rng), &"_1".to_string());
    insert_person(&mut vars, gen_person(&mut rng), &"_2".to_string());
    insert_person(&mut vars, gen_person(&mut rng), &"_3".to_string());
    strfmt(&CONTENTS, &vars).unwrap()
}
