fn main() {
  let input = include_str!("../input1.txt");
  let output = process(input);
  dbg!(output);
}

fn process(input: &str) -> String {
  let values = [("one", "eno"), ("two", "owt"), ("three", "eerht"), ("four", "ruof"), ("five", "evif"), ("six","xis"), ("seven", "neves"), ("eight","thgie"), ("nine", "enin") ];

  let output = input
    .lines()
    .map(|line| {
      let mut it = line.chars().map_filter
    })
}



