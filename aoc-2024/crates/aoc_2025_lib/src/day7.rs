use std::fs;

/// Trait to concatenate two u64 values.
trait Concat {
    fn concat(self, other: u64) -> u64;
}

/// Implementation of the Concat trait for u64.
impl Concat for u64 {
    /// Concatenates two u64 values.
    fn concat(self, other: u64) -> u64 {
        /// Counts the number of digits in a u64 value.
        fn count_digits(val: u64) -> u32 {
            let mut res = 1;
            let mut val = val/10;
            while val != 0 {
                res += 1;
                val /= 10;
            }
            res
        }

        let other_digits = count_digits(other);
        self * 10_u64.pow(other_digits) + other
    }
}

/// Returns true if the test is calibrated, false otherwise. A test is calibration if there
/// exist an equation evaluated from left to right using between the test paramters, 
/// using a combination of the * and + operators
fn is_calibrated(test_value: u64, test_parameters: Vec<u64>) -> bool {
    // Solution for this problem is essentially a depth first search on
    // a binary tree. Each node in the tree is the cumulative result of the
    // equation with the left and right child being the result of multiplying or
    // adding the value of the node to the next parameter.
    //
    // Using a stack data structure to solve this with each element in the stack being
    // a tuple the depth in the tree and the value of the
    let mut stack: Vec<(usize, u64)> = Vec::new();

    // Push left most param to stack.
    stack.push((0, test_parameters[0]));

    while let Some(top) = stack.pop() {
        let index = top.0 + 1;
        let cumulative_value = top.1;
        if index < test_parameters.len() {
            let right_param = test_parameters[index];
            stack.push((index, cumulative_value * right_param));
            stack.push((index, cumulative_value + right_param));
            stack.push((index, cumulative_value.concat(right_param)));
        } else {
            if cumulative_value == test_value {
                return true;
            }
        }
    }
    false
}

pub fn main(file_path: String) {
    let content = fs::read_to_string(file_path).unwrap();
    let mut calibrated_sum = 0;

    for line in content.lines() {
        let equation_components: Vec<&str> = line.split(": ").collect();
        let test_value = u64::from_str_radix(equation_components[0], 10).unwrap();
        let test_parameters: Vec<u64> = equation_components[1]
            .split(" ")
            .map(|param| u64::from_str_radix(param, 10).unwrap())
            .collect();

        if is_calibrated(test_value, test_parameters) {
            calibrated_sum += test_value;
        }
    }

    println!("Calibrated Sum: {calibrated_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenation_terst() {
        assert!(20_u64.concat(55) == 2055);
        assert!(37473_u64.concat(93282) == 3747393282);
        assert!(0_u64.concat(0) == 0);
        assert!(1_u64.concat(0) == 10);
    }
}
