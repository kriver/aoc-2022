from typing import List


def load(filename) ->List[str]:
    with open('data/' + filename, 'r') as f:
        lines = f.read().splitlines()
    return lines


def as_int(l: List[str]) -> List[int]:
    return list(map(lambda x: int(x), l))
