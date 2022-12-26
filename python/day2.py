from util import load

P1 = {'A': 1, 'B': 2, 'C': 3}
P2 = {'X': 1, 'Y': 2, 'Z': 3}

# win-loss (columns rock/paper/scissor)
WL = [
    [3, 6, 0],  # rock
    [0, 3, 6],  # paper
    [6, 0, 3],  # scissors
]

# choice-result (columns lose/draw/win)
CR = [
    [3, 1, 2],  # rock
    [1, 2, 3],  # paper
    [2, 3, 1],  # scissors
]


def score(abc, xyz):
    return xyz + WL[abc - 1][xyz - 1]


def part1(games):
    return sum([score(*g) for g in games])


def choose(abc, xyz):
    return (xyz - 1) * 3 + CR[abc - 1][xyz - 1]


def part2(games):
    return sum([choose(*g) for g in games])


def main():
    lines = load('day2.txt')
    games = []
    for line in lines:
        moves = line.split()
        games.append((P1[moves[0]], P2[moves[1]]))

    total = part1(games)
    print(f'Part 1: {total}')
    assert total == 11873

    total = part2(games)
    print(f'Part 2: {total}')
    assert total == 12014


if __name__ == "__main__":
    main()
