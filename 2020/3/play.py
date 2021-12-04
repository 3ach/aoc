import sys

pattern = [line.strip() for line in sys.stdin.readlines()]

def part1(pattern):
    rowlen = len(pattern[0])
    run = 3

    count = 0
    c = 0
    for r in range(len(pattern)):
        if pattern[r][c % rowlen] == "#":
            count += 1

        c += run

    print(count)

def part2(pattern):
    rowlen = len(pattern[0])
    runs = [1, 3, 5, 7, 1]
    rises = [1, 1, 1, 1, 2]

    counts = [0 for _ in runs]
    cs = [0 for _ in runs]
    for r in range(len(pattern)):
        for idx, rise in enumerate(rises):
            if r % rise != 0:
                continue

            if pattern[r][cs[idx] % rowlen] == "#":
                counts[idx] += 1

            cs[idx] += runs[idx]

    product = 1
    for count in counts:
        product *= count

    print(product)

part1(pattern)
part2(pattern)
