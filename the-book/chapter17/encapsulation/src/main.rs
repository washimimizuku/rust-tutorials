use encapsulation::AverageCollection;

fn main() {
    let mut average_collection = AverageCollection::new();

    println!("{:?}", average_collection);
    println!("Average: {}", average_collection.average());

    average_collection.add(1);
    average_collection.add(2);
    average_collection.add(3);
    println!("{:?}", average_collection);
    println!("Average: {}", average_collection.average());

    average_collection.remove();
    println!("{:?}", average_collection);
    println!("Average: {}", average_collection.average());

    average_collection.remove();
    average_collection.remove();
    println!("{:?}", average_collection);
    println!("Average: {}", average_collection.average());

    average_collection.add(545);
    average_collection.add(3454);
    average_collection.add(456546);
    println!("{:?}", average_collection);
    println!("Average: {}", average_collection.average());
}
