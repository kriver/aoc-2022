from util import load


def parse_lines(it, cur_dir):
    while True:
        line = next(it, None)
        if line:
            token = line.split(' ')
            if '$' == token[0]:
                if 'cd' == token[1]:
                    if '..' == token[2]:
                        break
                    else:
                        parse_lines(it, cur_dir[token[2]][1])
                elif 'ls' == token[1]:
                    pass  # ignore
            elif 'dir' == token[0]:
                cur_dir[token[1]] = ['D', {}]
            else:
                cur_dir[token[1]] = ['F', int(token[0])]
        else:
            break


def parse_input(lines):
    fs = {'/': ['D', {}]}  # initialize with root
    it = iter(lines)
    parse_lines(it, fs)
    return fs


def calculate_size(fs, limit):
    total_sz = 0
    this_sz = 0
    for f in fs.values():
        if f[0] == 'F':
            this_sz += f[1]
        else:  # it's a 'D'
            total, sub = calculate_size(f[1], limit)
            f.append(sub)
            total_sz += total + (sub if sub <= limit else 0)
            this_sz += sub
    return total_sz, this_sz


def find_to_delete(fs, at_least, current_sz):
    for f in fs.values():
        if f[0] == 'D':
            sz = f[2]
            if at_least <= sz < current_sz:
                current_sz = sz
            current_sz = find_to_delete(f[1], at_least, current_sz)
    return current_sz


def part1(fs):
    sz_limit = 100000
    return calculate_size(fs, sz_limit)[0]


def part2(fs):
    disk_sz = 70000000
    free_sz_required = 30000000
    used_sz = fs['/'][2]
    assert used_sz == 43441553
    min_delete_sz = free_sz_required - (disk_sz - used_sz)
    return find_to_delete(fs, min_delete_sz, disk_sz)


def main():
    # lines = load('day7-test.txt')
    # sz = part1(parse_input(lines))
    # assert sz == 95437

    lines = load('day7.txt')
    fs = parse_input(lines)
    sz = part1(fs)
    print(f'Total size of -100K directories: {sz}')
    assert sz == 1844187

    sz = part2(fs)
    print(f'Freeing up a directory of size: {sz}')
    assert sz == 4978279


if __name__ == "__main__":
    main()
