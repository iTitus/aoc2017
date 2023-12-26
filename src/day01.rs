use aoc_runner_derive::aoc;

fn solve_captcha(s: &str, offset_generator: impl Fn(i64) -> i64) -> u64 {
    let bs = s.trim().as_bytes();
    let l = bs.len() as i64;
    let offset = offset_generator(l);

    let mut result = 0;
    for i in 0..l {
        let a = bs[i as usize];
        let b = bs[(i + offset).rem_euclid(l) as usize];
        if a == b {
            result += (a - b'0') as u64;
        }
    }

    result
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    solve_captcha(input, |_| 1)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    solve_captcha(input, |l| l / 2)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("1122"), 3);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("1111"), 4);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1("1234"), 0);
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2("1212"), 6);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("1221"), 0);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2("123425"), 4);
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part2("123123"), 12);
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(part2("12131415"), 4);
    }
}
