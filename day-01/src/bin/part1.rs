fn main() {
    let input = include_str!("../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("msg");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input));
    }
}
