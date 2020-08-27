use std::io;

fn main() {
    println!("Simple Calculator!");
    println!("please input first number.");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("failed to scan your input number.");
    let first_number: i32 = first_number.trim().parse().expect("Invalid number.");
    println!("first inputted number is {}", first_number);

    println!("next, please input second number.");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("failed to scan your input number.");
    let second_number: i32 = second_number.trim().parse().expect("Invalid number");
    println!("second inpputed number is {}", second_number);

    println!("finally, choose operator.\n you can choose from +, -, *, /.");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("failed to scan your input operator.");
}
