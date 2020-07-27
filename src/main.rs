use phf::phf_map;
use std::io::{self, Write};

static PRECEDENCE: phf::Map<char, u8> = phf_map! {
    '+' => 1,
    '-' => 1,
    '*' => 2,
    '/' => 2,
    //'^' => 3,
};

fn calculate(expression: &Vec<String>) -> f64 {
    let mut stack = Vec::<f64>::new();

    for term in expression {
        if term.parse::<f64>().is_ok() {
            stack.push(term.parse().unwrap());
        } else if term.len() == 1 {
            let operation = &term.chars().last().unwrap();
            if PRECEDENCE.contains_key(operation) {
                if stack.len() < 2 {
                    return std::f64::NAN;
                }
                let oper2 = stack.pop().unwrap();
                let oper1 = stack.pop().unwrap();
                stack.push(do_operation(oper1, oper2, operation));
            }
        }
    }

    stack.pop().unwrap()
}

fn do_operation(oper1: f64, oper2: f64, operation: &char) -> f64 {
    match operation {
        '+' => oper1 + oper2,
        '-' => oper1 - oper2,
        '*' => oper1 * oper2,
        '/' => oper1 / oper2,
        //'^' => oper1.powf(oper2),
        _ => {
            println!("undefined operator '{}'", operation);
            std::f64::NAN
        }
    }
}

fn get_next_expression() -> String {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read expression");

    expression.trim().to_string()
}

fn to_postfix(expression: &String) -> String {
    let mut stack = vec!['('];
    let mut postfix = Vec::<char>::new();

    // remove all spaces
    let expression = {
        let mut s = String::from(expression) + ")";
        s.retain(|c| c != ' ');
        s
    };

    for c in expression.chars() {
        if c == '(' {
            stack.push(c);
        } else if c.is_alphanumeric() {
            postfix.push(c);
        } else if PRECEDENCE.contains_key(&c) {
            postfix.push(' ');
            let mut last = *stack.last().unwrap();
            while PRECEDENCE.contains_key(&last) && PRECEDENCE[&last] >= PRECEDENCE[&c] {
                postfix.push(stack.pop().unwrap());
                postfix.push(' ');
                last = *stack.last().unwrap();
            }

            stack.push(c);
        } else if c == ')' {
            while *stack.last().unwrap() != '(' {
                postfix.push(' ');
                postfix.push(stack.pop().unwrap());
            }
            stack.pop(); // Remove opening paren
        }
    }

    postfix.iter().collect()
}

fn tokenize_expression(expression: &String) -> Vec<String> {
    let tokenized_expression;
    let expression = to_postfix(expression);
    tokenized_expression = {
        let mut expr = Vec::<String>::new();
        let tokens = expression.split(" ");
        for token in tokens {
            expr.push(token.into());
        }
        expr
    };
    tokenized_expression
}

fn main() {
    let mut expression_history: Vec<String> = Vec::new();

    loop {
        // Get next expression
        print!("[{}]: ", expression_history.len());
        io::stdout().flush().unwrap();
        expression_history.push(get_next_expression());
        let expression = expression_history.last().unwrap();

        // Handle exit requests
        if expression == "exit" || expression == "quit" || expression == "q" || expression == "e" {
            break;
        }

        // Handle expression
        let expression = tokenize_expression(expression);
        let answer = calculate(&expression);
        println!("[{}]: {}\n", expression_history.len() - 1, answer);
    }
}
