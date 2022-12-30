from functools import reduce
from typing import Set, List

from util import load


def chunk(rucksacks: List[str], n: int) -> List[List[Set[str]]]:
    chunks = []
    for i in range(0, len(rucksacks), n):
        chunks.append([set(r) for r in rucksacks[i:i + 3]])
    return chunks


def split_in_halves(rucksack) -> List[Set[str]]:
    half = len(rucksack) // 2
    return [set(rucksack[:half]), set(rucksack[half:])]


def common_part(*args) -> str:
    return reduce(lambda s1, s2: s1.intersection(s2), *args).pop()


def priority(item: str) -> int:
    if item.isupper():
        return 27 + ord(item) - ord('A')
    else:
        return 1 + ord(item) - ord('a')


def part1(rucksacks: List[str]) -> int:
    return sum(
        map(priority,
            map(common_part,
                map(split_in_halves, rucksacks))))


def part2(rucksacks: List[str]) -> int:
    return sum(
        map(priority,
            map(common_part,
                chunk(rucksacks, 3))))


def main():
    rucksacks = load('day3.txt')

    sum1 = part1(rucksacks)
    print(f'Part 1: {sum1}')
    assert sum1 == 7674

    sum2 = part2(rucksacks)
    print(f'Part 2: {sum2}')
    assert sum2 == 2805


if __name__ == "__main__":
    main()
