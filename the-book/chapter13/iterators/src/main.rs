fn main() {
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    // Returns an iterator
    // v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Add 1 to all elements: {:?}", v2);

    assert_eq!(v2, vec![2, 3, 4]);
}
