import sys

expenses = [int(line.strip()) for line in sys.stdin.readlines()]
expenses.sort()

num_expenses = len(expenses)


def do1(expenses):
    start = 0
    end = num_expenses - 1

    for sidx in range(start, end):
        for lidx in range(end, start, -1):
            small = expenses[sidx]
            large = expenses[lidx]

            if small + large == 2020:
                print(small * large)
                return
            elif small + large > 2020:
                end -= 1
            else:
                break

def do2(expenses):
    sexpenses = set(expenses)
    start = 0
    end = num_expenses - 1

    for sidx in range(start, end):
        for lidx in range(end, start, -1):
            small = expenses[sidx]
            large = expenses[lidx]


            if small + large >= 2020:
                end -= 1
            else:
                diff = 2020 - (small + large)
                if diff < small or diff > large:
                    continue

                if diff in sexpenses:
                    print(diff * small * large)


do2(expenses)
