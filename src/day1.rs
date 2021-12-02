//! --- Day 1: Sonar Sweep ---
//! [Part 1](https://adventofcode.com/2021/day/1#part1)
//! [Part 2](https://adventofcode.com/2021/day/1#part2)

#[allow(dead_code)]
fn part1(input: &Vec<usize>) -> usize {
    let mut increase_count = 0;
    let mut idx = 1;

    if input.len() < 2 {
        return increase_count;
    }

    while idx < input.len() {
        if input[idx] > input[idx - 1] {
            increase_count += 1
        }
        idx += 1
    }

    increase_count
}

#[allow(dead_code)]
fn part2(input: &Vec<usize>) -> usize {
    let window_3_sum = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>();
    part1(&window_3_sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    fn get_input(file_path: &str) -> Vec<usize> {
        read_to_string(file_path)
            .expect(&format!("{} file path shoul exist", file_path))
            .lines()
            .map(|x| x.parse::<usize>().expect("All values should be ints"))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part1() {
        let input = get_input("test/day1_input.txt");
        assert_eq!(part1(&input), 1121);
    }

    #[test]
    fn test_part2() {
        let input = get_input("test/day1_input.txt");
        assert_eq!(part2(&input), 1065);
    }
}
