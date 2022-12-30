import re
from typing import Tuple

from util import load

SectionRange = Tuple[int, int]


def parse_line(line: str) -> Tuple[SectionRange, SectionRange]:
    num = [int(n) for n in re.split(r'[,-]', line)]
    return (num[0], num[1]), (num[2], num[3])


def full_overlap(sr1, sr2) -> bool:
    return (sr1[0] <= sr2[0] and sr2[1] <= sr1[1]) or (sr2[0] <= sr1[0] and sr1[1] <= sr2[1])


def any_overlap(sr1, sr2) -> bool:
    return (sr1[0] <= sr2[0] <= sr1[1]) or \
        (sr1[0] <= sr2[1] <= sr1[1]) or \
        (sr2[0] <= sr1[0] <= sr2[1]) or \
        (sr2[0] <= sr1[1] <= sr2[1])


def part1(pairs):
    return len([pair for pair in pairs if full_overlap(*pair)])


def part2(pairs):
    return len([pair for pair in pairs if any_overlap(*pair)])


def main():
    lines = load('day4.txt')
    pairs = [parse_line(line) for line in lines]

    count1 = part1(pairs)
    print(f'Part 1: {count1}')
    assert count1 == 562

    count2 = part2(pairs)
    print(f'Part 2: {count2}')
    assert count2 == 924


if __name__ == "__main__":
    main()
