use std::fs;

static INPUT_DATA: &str = "../data/input";
//static INPUT_DATA: &str = "../data/test_input";

fn main() {
    let data = fs::read_to_string(INPUT_DATA).unwrap();
    for id in String::from(data).split(",") {
        println!("ID: {}", id);
        check_ids(id);
    }
}

fn check_ids(id: &str) -> i64 {
    let string_id = String::from(id);
    let mut ids = string_id.split("-");
    let id1 = ids.next().unwrap();
    let id2 = ids.next().unwrap();
    println!("ID 1: {}, ID 2: {}", id1, id2);
    0
}




