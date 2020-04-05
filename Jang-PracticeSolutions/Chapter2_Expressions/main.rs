use std::io;
use std::ops::Mul;

    // loop {
    //    println!("Hello, world!"); 
    // }
    
    // // Prints Table of the input.
    // println!("Input a number:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input);

    // let input:i32 = input.trim().parse().unwrap();
    // for i in 1..11{
    //     let table = i * input;
    //     println!("{} x {} = {}", input, i, table);
    // }

    // let k = 2*5 - 2;
    // for i in 0..(5) {
    //     for j in 0..k {
    //         println!("\n");
    //         let k = k - 1;
    //         for j in 0..(i+1) {
    //             println!("*\n")
    //         }
    //         println!("\n");

    //     }
    // }
    
    //input
    // let mut input = String::new();
    // io::stdin().read_line(&mut input);
    // let input:i32 = input.trim().parse().unwrap();
    
//    //function for even_or_odd
//     println!("{}",even_or_odd(input));

//     fn even_or_odd(num:i32) -> String {
        
//     if num % 2 == 0 {
//         return "Even".to_string();
//     }
//     else {
//         "Odd".to_string()
//     }
//     }
    
// fn factorial(n:i32) -> i32 {
//     if n == 1 || n == 0 {
//         return 1;
//     }
//     else {
//         return n * factorial(n-1);
//     }
// }

// println!("{}! = {}",input,factorial(input));

// match input {
//     1 => println!("one"),
//     2 => println!("two"),
//     _ => println!("NaN"), 
// }

//Diverging Function
// fn diverging_function(x:i32, y:i32) -> ! {
//     let z = x + y;
//     panic!("{}",z);
// }

// diverging_function(1, 2);

// // Leap Year Program
// fn main() {
//     let year = 1997;
//     if LeapYear(year) == true {
//         println!("Leap Year");
//     }
//     else {
//         println!("Not a Leap Year");
//     }
// }

// fn LeapYear(year:i32) -> bool {
//     if year % 400 == 0 {
//         return true;
//     }
//     else if year % 100 == 0 {
//         return false;
//     }
//     else if year % 4 == 0 {
//         return true;
//     }
//     else {
//         return false;
//     }
// }

// use std::mem;
// fn main() {
//     swap_numbers(4, 10);
// }

// fn swap_numbers(a:i32,b:i32) {
//     let mut x = a;
//     let mut y = b;  
//     let swap = mem::swap(&mut x, &mut y );
//     println!("Swapped: ({},{})",x,y);
// }

// // Finding Prime Numbers
// fn main() {
//     println!("{}", is_Prime(25));
// }

// fn is_Prime(n:i32) -> bool {
    
//     for i in 2..n {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     return true;
// }

// fn main() {
//     let mut list = [5,3,1,4,2];
//     list.sort();
//     println!("{:?}", list);
// }

// fn main() {
//     let vec1 = vec![1, 2, 3];
//     let vec2 = vec![4, 5, 6];

//     let mut iter = vec1.iter().zip(vec2.iter());
//     println!("{:?}", iter);

// }
