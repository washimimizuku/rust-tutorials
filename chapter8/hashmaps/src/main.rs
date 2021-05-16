fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    // With iterators
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50, 100];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Scores: {:?}", scores);

    // String values moved into the hash map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at
    // this point.

    // Accessing values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score: {:?}", score);

    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Scores overwritten: {:?}", scores);

    // Insert if Empty
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Scores: {:?}", scores);

    // Update a value based on the old values
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
