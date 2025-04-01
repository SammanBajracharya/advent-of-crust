mod part_1;
mod part_2;

pub use part_2::part_2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = String::from("./src/day_5/test.ts");
        part_2(file_path);
    }
}
