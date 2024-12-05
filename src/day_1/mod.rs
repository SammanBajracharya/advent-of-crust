mod part_1;
mod part_2;
mod quicksort;

#[allow(unused_imports)]
use part_1::part_1;
#[allow(unused_imports)]
use part_2::part_2;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_file_path = String::from("./src/day_1/test.txt");
        let result_1 = part_1(test_file_path);

        let input_file_path = String::from("./src/day_1/input.txt");
        let result_2 = part_1(input_file_path);

        assert_eq!(result_1, 11);
        assert_eq!(result_2, 1660292);
    }

    #[test]
    fn test_part_2() {
        let test_file_path = String::from("./src/day_1/test.txt");
        let result_1 = part_2(test_file_path);

        let input_file_path = String::from("./src/day_1/input.txt");
        let result_2 = part_2(input_file_path);

        assert_eq!(result_1, 31);
        assert_eq!(result_2, 22776016);
    }
}

