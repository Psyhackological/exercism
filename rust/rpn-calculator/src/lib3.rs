#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_vec: Vec<i32> = Vec::new();
    for calculator_input in inputs {
        use CalculatorInput::*;
        match calculator_input {
            Value(v) => num_vec.push(*v),
            _ => {
                if let (Some(s), Some(f)) = (num_vec.pop(), num_vec.pop()) {
                    match calculator_input {
                        Add => num_vec.push(f + s),
                        Subtract => num_vec.push(f - s),
                        Multiply => num_vec.push(f * s),
                        Divide => match s {
                            0 => return None,
                            _ => num_vec.push(f / s),
                        },
                        _ => (),
                    }
                } else {
                    return None;
                }
            }
        }
    }
    if num_vec.len() == 1 {
        num_vec.pop()
    } else {
        None
    }
}
