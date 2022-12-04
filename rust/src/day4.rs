use crate::util::load;

type Pair = Vec<u32>;

fn load_pairs(filename: &str) -> Vec<Pair> {
    load::<String>(filename)
        .into_iter()
        .map(|l| {
            l.split(&['-', ','])
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1() -> usize {
    let pairs: Vec<Pair> = load_pairs("data/day4.txt");
    pairs
        .into_iter()
        .filter(|p| (p[0] <= p[2] && p[1] >= p[3]) || (p[0] >= p[2] && p[1] <= p[3]))
        .count()
}

pub fn part2() -> usize {
    let pairs: Vec<Pair> = load_pairs("data/day4.txt");
    pairs
        .into_iter()
        .filter(|p| {
            (p[2] <= p[0] && p[0] <= p[3])
                || (p[2] <= p[1] && p[1] <= p[3])
                || (p[0] <= p[2] && p[2] <= p[1])
                || (p[0] <= p[3] && p[3] <= p[1])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let count = part1();
        println!("Fully contains count {}", count);
        assert_eq!(count, 562);
    }

    #[test]
    fn test_part2() {
        let count = part2();
        println!("Overlap count {}", count);
        assert_eq!(count, 924);
    }
}
