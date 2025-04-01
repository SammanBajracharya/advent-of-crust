mod part_1;

pub use part_1::part_1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = String::from("./src/day_5/test.ts");
        part_1(file_path);
    }
}
