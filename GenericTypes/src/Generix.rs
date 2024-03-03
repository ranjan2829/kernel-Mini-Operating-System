struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!(" p.x ,p.y ====>>>>>>>>{},{}", p.x, p.y);
}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };

//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> usize {
//     let mut largest_index = 0;

//     for (index, item) in list.iter().enumerate() {
//         if *item > list[largest_index] {
//             largest_index = index;
//         }
//     }

//     largest_index
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let largest_index = largest(&number_list);
//     println!("The largest number is {}", number_list[largest_index]);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let largest_index = largest(&char_list);
//     println!("The largest char is {}", char_list[largest_index]);
// }
