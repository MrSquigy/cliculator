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
        } else if term.len() == 1 && PRECEDENCE.contains_key(&term.chars().last().unwrap()) {
            let operation = &term.chars().last().unwrap();
            if stack.len() < 2 {
                return std::f64::NAN;
            }
            let oper2 = stack.pop().unwrap();
            let oper1 = stack.pop().unwrap();
            stack.push(do_operation(oper1, oper2, operation));
        } else {
            println!("Undefined term '{}'", term);
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
        } else if c.is_alphanumeric() || c == '$' {
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
    let mut expr = Vec::<String>::new();
    let tokens = expression.split(" ");
    for token in tokens {
        expr.push(token.into());
    }
    expr
}

fn insert_vars(expression: &Vec<String>, history: &Vec<(String, f64)>) -> Vec<String> {
    let mut expr = Vec::new();

    for term in expression {
        if term.starts_with("$") {
            let idx = term.get(1..term.len()).unwrap();
            let idx = idx.parse::<usize>().unwrap();
            expr.push(history[idx].1.to_string());
        } else {
            expr.push(term.clone());
        }
    }

    expr
}

fn main() {
    let mut expression_history = Vec::<(String, f64)>::new();

    loop {
        // Get next expression
        print!("[{}]: ", expression_history.len());
        io::stdout().flush().unwrap();
        let expression_text = get_next_expression();

        // Handle exit requests
        match expression_text.to_lowercase().as_str() {
            "exit" | "e" | "quit" | "q" => break,
            _ => {}
        }

        // Handle expression
        let expression = to_postfix(&expression_text);
        let expression = tokenize_expression(&expression);
        let expression = insert_vars(&expression, &expression_history);
        let answer = calculate(&expression);
        expression_history.push((expression_text, answer));
        println!("[{}]: {}\n", expression_history.len() - 1, answer);
    }
}
