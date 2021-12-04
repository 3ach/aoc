import sys

lines = sys.stdin.readlines()
groups = []
group = []

for line in lines:
    line = line.strip()
    if line == "":
        groups.append(group)
        group = []
        continue

    group.append([x for x in line])

groups.append(group)

def part1(groups):
    sum = 0
    for group in groups:
        answers = set()
        for player in group:
            answers.update(question for question in player)
        sum += len(answers)

    print(sum)

def part2(groups):
    sum = 0

    for group in groups:
        answers = set(question for question in group[0])
        for player in group[1:]:
            answers = answers & set(question for question in player)

        sum += len(answers)

    print(sum)

part2(groups)
