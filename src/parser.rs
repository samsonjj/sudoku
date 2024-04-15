pub fn parse(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            if line.len() != 9 {
                panic!("line did not contain 9 characters");
            }

            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("character could not be parsed to digit")
                })
                .collect()
        })
        .collect()
}
