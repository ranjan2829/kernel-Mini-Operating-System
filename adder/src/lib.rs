// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//     #[test]
//     fn another() {
//         panic!("Make this Test Fail ");
//     }
// }
// #[derive(Debug)]
// struct tect {
//     width: u32,
//     height: u32,
// }
// impl rect {
//     fn hold(&self, other: &rect) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn larger() {
//         let large = rect {
//             width: 8,
//             height: 9,
//         };
//         let small = rect {
//             width: 3,
//             height: 4,
//         };
//         assert!(large.hold(&small));
//     }
// }
//
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
//}
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
//}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         // --snip--
//         let larger = Rectangle {
//         width: 8,
//         height: 7,
//     };
//     let smaller = Rectangle {
//         width: 5,
//         height: 1,
//     };

//     assert!(larger.can_hold(&smaller));
// }

// #[test]
// fn smaller_cannot_hold_larger() {
//     let larger = Rectangle {
//         width: 8,
//         height: 7,
//     };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }
