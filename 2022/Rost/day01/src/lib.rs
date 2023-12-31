pub fn process_part1(input: &str) -> String {
    let split_input = input.split("\n");
    let mut sum = 0;
    let mut max = 0;
    for i in split_input {
        if i.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            let i: u32 = i.parse().expect("Not a number");
            sum += i;
        }
    }
    max.to_string()
}

pub fn process_part1_better(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut input = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    input.sort_by(|a, b| b.cmp(a));
    let result: u32 = input.iter().take(3).sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part1(input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part_1_better_test() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part1_better(input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part_2_test() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part2(input);
        assert_eq!(result, "45000");
    }
}
