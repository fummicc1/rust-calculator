use std::io;

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide
}

impl Operator {
    fn initialize(operator: &str) -> Operator {
        match operator {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!()
        }
    }

    // i32はCopyトレイトに適合しているので参照でなくても大丈夫
    fn calclate(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Minus => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs
        }
    }
}

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
    let mut input_operator = String::new();
    io::stdin().read_line(&mut input_operator).expect("failed to scan your input operator.");
    let input_operator = input_operator.trim();
    let operator: Operator = Operator::initialize(&input_operator);
    let result = operator.calclate(first_number, second_number);
    println!("Result is {}", result);
}
