  // #[derive(Debug)]
// struct info {
//     name: String,
//     age: u32,
//     roll_no: u32,
//     marks: u64,
// }
// fn main() {

//     let stdt1 = info {
//         name: String::from("Alex Hales"),
//         age: 20,
//         roll_no: 142,
//         marks: 820,
//     };
//     println!("{:?}", stdt1);
// }

use std::io;
#[derive(Debug)]

struct Date {
    day: isize,
    month: String,
    year: isize,
}

fn main() {

    // Taking Input as date.
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input:isize = input.trim().parse().unwrap();

    // Taking Input as String.
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);

    // Taking Input as year.
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let input3:isize = input.trim().parse().unwrap();

    let date = Date {
        day: input,
        month: String::from(input2),
        year: input,
    };

    println!("{:?}", date);


}
