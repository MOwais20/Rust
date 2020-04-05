use std::io;
fn main() {
    // Hello, world program
    println!("Hello, world!\n");

    // Printing F letter on terminal.
    println!("######");
    println!("#\n#\n#####\n#\n#\n#\n");

    let tuple = (300,98.6);
    println!("{}\n", tuple.1);
    
    let array = [4,8,12,16];
    println!("{}\n", array[0]);

    println!("Whats your name?  ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input:String = input.trim().parse().unwrap();
    println!("Hello, {}!\n", input);

    let sixteen = [0; 16];
    println!("{:?}", sixteen);

}

