mod part_1;
mod part_2;
mod quicksort;

#[allow(unused_imports)]
pub use part_1::part_1;
#[allow(unused_imports)]
pub use part_2::part_2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_part_1 = part_1(String::from("./src/day_2/test.txt"));
        let input_part_1 = part_1(String::from("./src/day_2/input.txt"));

        assert_eq!(test_part_1, 2);
        assert_eq!(input_part_1, 606);
    }

    #[test]
    fn test_part_2() {
        let test_part_2 = part_2(String::from("./src/day_2/test.txt"));
        let input_part_2 = part_2(String::from("./src/day_2/input.txt"));

        assert_eq!(test_part_2, 4);
        assert_eq!(input_part_2, 644);
    }
}
