#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    if inputs.len() == 0 { return None }
    for item in inputs {
        match item {
            CalculatorInput::Value(number) => stack.push(*number),            
            _ => {
                if stack.len() < 2 { return None }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                match item {
                    CalculatorInput::Add => {stack.push(a+b) ;}
                    CalculatorInput::Subtract => { stack.push(b - a); }
                    CalculatorInput::Multiply => { stack.push(a*b ); }
                    CalculatorInput::Divide => { stack.push(b/a);}
                    _ => return None
                }
            }
        }
    }
    if stack.len() != 1 { return None }
    return stack.pop();
}
