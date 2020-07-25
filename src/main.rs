use std::io::{self, Write};

fn calculate(expression: &Vec<&str>) -> f64 {
    0.0
}

fn get_next_expression() -> String {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read expression");

    return String::from(expression.trim());
}

fn tokenize_expression(expression: &String) -> Vec<&str> {
    let tokenized_expression: Vec<&str>;
    tokenized_expression = expression.split(" ").collect();

    // TODO: Implement a prefix notation tokenizer

    return tokenized_expression;
}

fn main() {
    let mut expression_history: Vec<String> = Vec::new();

    loop {
        print!("[{}]: ", expression_history.len());
        io::stdout().flush().unwrap();
        expression_history.push(get_next_expression());
        let expression = expression_history.last().unwrap();

        if expression == "exit" || expression == "quit" {
            break;
        }
        let expression = tokenize_expression(expression);
        println!("{:?}", expression);
    }
}
