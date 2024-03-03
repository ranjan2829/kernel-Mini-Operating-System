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
