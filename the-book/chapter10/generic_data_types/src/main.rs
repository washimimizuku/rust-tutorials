fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Same as both functions above, but generic
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is: {}", result);

    let result = largest(&char_list);
    println!("The largest char is: {}", result);

    // Struct with generic types
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0};
    println!("The integer point is: {:?}", integer);
    println!("The float point is: {:?}", float);

    // Struct with different generic types
    #[derive(Debug)]
    struct PointMultiple<T, U> {
        x: T,
        y: U,
    }

    let both_integer = PointMultiple { x: 5, y: 10};
    let both_float = PointMultiple { x: 1.0, y: 4.0};
    let integer_and_float = PointMultiple { x: 5, y: 4.0};
    println!("The both_integer point is: {:?}", both_integer);
    println!("The both_float point is: {:?}", both_float);
    println!("The integer_and_float point is: {:?}", integer_and_float);

    // Generic in method definitions
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // One type method definition (Method will only exist for Points of type f32)
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5.0, y: 10.0 };
    println!("p.distance_from_origin = {}", p.distance_from_origin());

    // Mixed generic in method definitions
    impl<T, U> PointMultiple<T, U> {
        fn mixup<V, W>(self, other: PointMultiple<V, W>) -> PointMultiple<T, W> {
            PointMultiple {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = PointMultiple { x: 5, y: 10.4 };
    let p2 = PointMultiple { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


}
