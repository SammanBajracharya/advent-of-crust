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
        let test_data_result = part_1(String::from("./src/day_4/test.txt"));
        let input_data_result = part_1(String::from("./src/day_4/input.txt"));

        assert_eq!(18, test_data_result);
        assert_eq!(2654, input_data_result);
    }

    #[test]
    fn test_part_2() {
        let test_data_result = part_2(String::from("./src/day_4/test.txt"));
        let input_data_result = part_2(String::from("./src/day_4/input.txt"));

        assert_eq!(9, test_data_result);
        assert_eq!(1990, input_data_result);
    }
}
