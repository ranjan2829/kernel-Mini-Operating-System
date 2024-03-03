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
} // struct Point<T> {
  //     x: T,
  //     y: T,
  // }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }
//rgest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }

// fn Largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = Largest(&number_list);
//     println!("largest is {}", result);
//     let num_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 99];
//     let ans = Largest(&num_list);
//     println!("largest of 2nd list ::::====>{} ", ans);
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }
