use std::collections::HashMap;

fn main() { 
let mut scores = HashMap::new();

scores.insert(String::from("Yellow"), 50);
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Green")).or_insert(50);

let team_name = String::from("Blue");


for (key, value) in &scores {
    println!("{key}: {value}");
}

println!("{:?}", scores);

}