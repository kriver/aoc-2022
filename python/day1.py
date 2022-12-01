from util import as_int, load


def part1(calories):
    most = 0
    current = 0
    for cal in calories:
        if cal == '':
            most = max(current, most)
            current = 0
        else:
            current += int(cal)
    return most


def part2(calories):
    top3 = [0, 0, 0]
    current = 0
    for cal in calories:
        if cal == '':
            for i in range(3):
                if current > top3[i]:
                    top3.insert(i, current)
                    break
            current = 0
        else:
            current += int(cal)
    return sum(top3[0:3])


def main():
    calories = load('day1.txt')

    most = part1(calories)
    print(f'Part 1: {most}')
    assert most == 72017

    top3 = part2(calories)
    print(f'Part 2: {top3}')
    assert top3 == 212520


if __name__ == "__main__":
    main()
