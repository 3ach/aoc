import sys

passes = sys.stdin.readlines()

def part1(passes):
    max = -1
    for passe in passes:
        binstr = passe.replace("F", "0").replace("L", "0").replace("B", "1").replace("R", "1")
        id = int(binstr, 2)

        if id > max:
            max = id

    print(max)

def part2(passes):
    seats = []
    for passe in passes:
        binstr = passe.replace("F", "0").replace("L", "0").replace("B", "1").replace("R", "1")
        id = int(binstr, 2)

        seats.append(id)

    seats.sort()

    for idx in range(len(seats) - 1):
        if seats[idx + 1] - seats[idx] > 1:
            print(seats[idx] + 1)

part1(passes)
part2(passes)
