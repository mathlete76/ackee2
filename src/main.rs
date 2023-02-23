// A command line calculator

use std::io;

fn main() {

    println!("Enter an expression with space between each argument and operators\nType 'quit' to exit.");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "quit" {
            break;
        }
        let result = eval(&input);
        println!("{}", result);
    }
}

fn eval(input: &str) -> f64 {
    let mut result = 0.0;
    let mut op = '+';
    for token in input.split_whitespace() {
        match token {
            "+" => op = '+',
            "-" => op = '-',
            "*" => op = '*',
            "/" => op = '/',
            "%" => op = '%',
            "^" => op = '^',
            n => {
                let n = n.parse::<f64>().unwrap();
                match op {
                    '+' => result += n,
                    '-' => result -= n,
                    '*' => result *= n,
                    '/' => result /= n,
                    '%' => result %= n,
                    '^' => result = result.powf(n),
                    _ => unreachable!(),
                }
            }
        }
    }
    result
}