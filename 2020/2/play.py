import sys
import re

problem = [line.split(": ") for line in sys.stdin.readlines()]
print(len(problem))

def part1(problem):
    count = 0

    for criteria, password in problem:
        rang, letter = criteria.split()
        non_letter_fragment = f"[^{letter}]*"
        letter_fragment = f"{letter}"

        min, max = [int(x) for x in rang.split("-")]

        full_regex = non_letter_fragment
        for _ in range(min):
            full_regex += letter_fragment + non_letter_fragment

        for _ in range(max - min):
            full_regex += letter_fragment + "?" + non_letter_fragment

        full_regex = f"^{full_regex}$"
        letter_count = password.count(letter)

        if re.match(full_regex, password):
            count += 1

            if letter_count < min or letter_count > max:
                print("PROBLEM!")
                print(criteria)
                print(password)
                print(full_regex)

    print(count)

def part2(problem):
    count = 0

    for criteria, password in problem:
        idxs, letter = criteria.split()
        pos1, pos2 = [int(idx) - 1 for idx in idxs.split("-")]

        pos1match = password[pos1] == letter
        pos2match = password[pos2] == letter

        if pos1match ^ pos2match:
            count += 1

    print(count)

part2(problem)
