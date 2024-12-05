mod part_1;
mod part_2;

#[allow(unused_imports)]
pub use part_1::part_1;
#[allow(unused_imports)]
pub use part_2::part_2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result_test_data = part_1(String::from("./src/day_3/test.txt"));
        let result_input_data = part_1(String::from("./src/day_3/input.txt"));

        assert_eq!(result_test_data, 161);
        assert_eq!(result_input_data, 153469856);
    }

    #[test]
    fn test_part_2() {
        let result_test_data = part_2(String::from("./src/day_3/test.txt"));
        let result_input_data = part_2(String::from("./src/day_3/input.txt"));

        assert_eq!(result_test_data, 48);
        assert_eq!(result_input_data, 77055967);
    }
}
