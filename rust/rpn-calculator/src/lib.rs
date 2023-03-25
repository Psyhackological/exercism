#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

// GPT-4 version
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_vec: Vec<i32> = Vec::new();

    for calculator_input in inputs {
        use CalculatorInput::*;

        match calculator_input {
            Value(v) => num_vec.push(*v),
            Add | Subtract | Multiply | Divide => {
                if num_vec.len() < 2 {
                    return None;
                }

                let s = num_vec.pop().unwrap();
                let f = num_vec.pop().unwrap();

                let result = match calculator_input {
                    Add => f + s,
                    Subtract => f - s,
                    Multiply => f * s,
                    Divide => {
                        if s == 0 {
                            return None;
                        }
                        f / s
                    }
                    _ => unreachable!(),
                };

                num_vec.push(result);
            }
        }
    }

    if num_vec.len() == 1 {
        num_vec.pop()
    } else {
        None
    }
}
