use std::{collections::HashMap, io::Write};
use value_unit::{self, ValueUnit, TryAdd, TrySub};

// These are all the operators in order
const OPERATOR_FUNCTIONS: [(char, fn(ValueUnit, ValueUnit) -> Result<ValueUnit, String>); 5] = [
    ('^', |left, right| {
        if right.value.abs() >= 1.0 {
            let pow = right.value.round() as i8;
            return Ok(left.pow(pow));
        } else {
            let pow = (1.0_f64 / right.value).round().rem_euclid(2f64.powi(32)) as i8;
            return Ok(left.root(pow));
        }
    }),
    ('/', |left, right| Ok(&left / &right)),
    ('*', |left, right| Ok(&left * &right)),
    ('-', |left, right| (&left).try_sub(&right)),
    ('+', |left, right| (&left).try_add(&right))
];



fn process_eq(variables: &HashMap<String, ValueUnit>, input: String) -> Result<ValueUnit, String> {
    // split equation into array of ValueUnits and an array of operators
    let mut values: Vec<ValueUnit> = Vec::new();
    let mut ops = Vec::new();
    let operators = "*+-/^";
    let mut brackets_sum: u8 = 0; // This keeps track of the opened brackets minus the closed ones
    let mut buffer = String::new();
    let mut was_previous_value_sub_eq = false;
    let mut previous_char = '0';
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        // This code deals with brackets and sub equations. Sub equations get evaluated first
        if c == ')' {
            if brackets_sum > 1 {
                buffer.push(c);
            }
            if brackets_sum < 1 {
                return Err("Unmatched closing bracket".to_string())
            }
            brackets_sum -= 1;
        } else if brackets_sum == 0 && operators.contains(c) && previous_char == ' ' && chars.peek() == Some(&' ') { // If we are outside of brackets and find an operator after a space
            if was_previous_value_sub_eq { // If the buffer is a sub eq, process it (recursion)
                values.push(process_eq(variables, buffer)?);
                was_previous_value_sub_eq = false;
            } else if variables.contains_key(buffer.trim()) {
                values.push(variables[buffer.trim()].clone());
            } else {
                values.push(buffer.try_into()?);
            }
            buffer = String::new();
            ops.push(c);
        } else if c == '(' {
            if brackets_sum > 0 {
                buffer.push(c);
            }
            brackets_sum += 1;
            was_previous_value_sub_eq = true;
        } else {
            buffer.push(c);
        }
        previous_char = c;
    }
    if was_previous_value_sub_eq {
        values.push(process_eq(variables, buffer)?);
    } else if variables.contains_key(buffer.trim()) {
        values.push(variables[buffer.trim()].clone());
    } else {
        values.push(buffer.try_into()?);
    }
    assert_eq!(ops.len() + 1, values.len());

	// This does all operations in order of operation due to the order of the list and left to right due to the position function
    for (operator, operator_function) in OPERATOR_FUNCTIONS {
        while ops.contains(&operator) {
            if let Some(index) = ops.iter().position(|c| c == &operator) {
                let left = values.remove(index);
                let right = values.remove(index);
                ops.remove(index);
                values.insert(index, operator_function(left, right)?)
            } else {
                unreachable!(); // We just detemined it does contain the operator
            }
        }
    }
    assert_eq!(values.len(), 1);
    return Ok(values[0].clone());
}

fn is_valid_var_name(var_name: &String) -> bool {
    let lower_var_name = var_name.to_lowercase();
    let mut var_name_iter = lower_var_name.bytes();
    if let Some(first_char) = var_name_iter.next() {
        if !(97..=122).contains(&first_char) {
            return false
        }
        for char in var_name_iter {
            if !(97..=122).contains(&char) && !(48..=57).contains(&char) && char != 95 { //any alphanumaric or underscore
                return false
            }
        }
        return true
    } else {
        return false;
    }
}

fn main() {
    let mut variables = HashMap::new();
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
		let input_str = input.trim();
        if input_str == "exit" {
            break;
        }
		if input_str == "" {
			continue;
		}
        match input_str.matches('=').count() {
            0 => {
                match process_eq(&variables, input_str.to_string()) {
                    Err(e) => println!("Error: {e}"),
                    Ok(res) => println!("{res}"),
                }
            },
            1 => {
                let mut var_name_and_var_eq = input_str.split('=');
                let var_name = var_name_and_var_eq.next().unwrap().trim().to_string();
                if !is_valid_var_name(&var_name) {
                    println!("Error: variable names must be alphanumeric and start with an alphabetic character");
                    continue;
                }
                let var_eq = var_name_and_var_eq.next().unwrap().trim().to_string();
                match process_eq(&variables, var_eq) {
                    Err(e) => println!("Error: {e}"),
                    Ok(res) => {
                        println!("{var_name} = {res}");
                        variables.insert(var_name, res.clone());
                    },
                }
            },
            _ => println!("Error: too many equals signs")
        }
    }
}
