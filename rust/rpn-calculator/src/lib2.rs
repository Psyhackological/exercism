#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

// Reformated version of first solution
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_vec: Vec<i32> = Vec::new();

    for calculator_input in inputs {
        use CalculatorInput::*;

        if let Value(v) = calculator_input {
            num_vec.push(*v);
        } else if let (Some(s), Some(f)) = (num_vec.pop(), num_vec.pop()) {
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
        } else {
            return None;
        }
    }

    if num_vec.len() == 1 {
        num_vec.pop()
    } else {
        None
    }
}
