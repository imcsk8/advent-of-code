use std::fs;
use std::collections::HashMap;

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
    let mut first_ids: HashMap<&str, bool> = HashMap::new();
    let mut last_ids: HashMap<&str, bool> = HashMap::new();
    let string_id = String::from(id);
    let mut ids = string_id.split("-");
    let id1 = ids.next().unwrap();
    let id2 = ids.next().unwrap();
    println!("ID 1: {}, ID 2: {}", id1, id2);
    first_ids.insert(id1, false);
    last_ids.insert(id2, false);
    println!("First ID hash: {:?} ", first_ids);
    println!("Last ID hash: {:?} ", last_ids);
    /*
    23170346 == 23170346 -> false
    2317034  == 2317034  -> false
    231703   == 231703 
    23170
    2317
    231
    23
    2
    */
    check_duplicates(id1);
    0
}

/*
* 6464
* 646
* 64
* 6
*
*/
/// Check for duplicate numbers in a string
fn check_duplicates(value: &str) -> bool {
    let mut ids: HashMap<String, i32> = HashMap::new();
    let mut control_ids: Vec<i32>;
    let mut complete = String::from("");
    let mut count = 0;

    for (i, c) in value.chars().enumerate() {
        let index = c.to_string();
        if ids.contains_key(&index) {
            let count = ids.get(&index).unwrap() + 1;
            ids.insert(index.clone(), count);
        } else {
            ids.insert(index.clone(), 1);
        }
        println!("Index: {}, Value {}", index, ids.get(&index).unwrap());
    }
    false
}

