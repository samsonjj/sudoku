pub fn parse(data: &str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|line| {
            if line.len() != 9 {
                panic!("line did not contain 9 characters");
            }

            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("character could not be parsed to digit")
                        as usize
                })
                .collect()
        })
        .collect()
}

pub fn parse_list(data: &str) -> Vec<Vec<Vec<usize>>> {
    data.lines()
        .collect::<Vec<&str>>()
        .chunk_by(|_line1, line2| !line2.starts_with("Grid"))
        .map(|chunk: &[&str]| {
            let chunk_str = chunk
                .iter()
                .skip(1)
                .map(|s: &&str| s.to_string())
                .collect::<Vec<String>>();
            parse(&chunk_str.join("\n"))
        })
        .collect()
}
