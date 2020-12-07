import sys
import re

passports = []
passport = {}

for line in sys.stdin.readlines():
    line = line.strip()
    if line == "":
        passports.append(passport)
        passport = {}

    for item in line.split():
        key, value = item.split(":")
        passport[key] = value

passports.append(passport)


def part1(passports):
    required = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

    count = 0
    for passport in passports:
        given = set(passport.keys())
        if len(required - given) == 0:
            count += 1

    print(count)


def part2(passports):
    count = 0
    for passport in passports:
        try:
            byr = int(passport["byr"])
            if byr < 1920 or byr > 2002:
                continue

            iyr = int(passport["iyr"])
            if iyr < 2010 or iyr > 2020:
                continue

            eyr = int(passport["eyr"])
            if eyr < 2020 or eyr > 2030:
                continue

            hgt = passport["hgt"]
            unit = hgt[-2:]
            measure = int(hgt[:-2])
            if unit == "in":
                if measure < 59 or measure > 76:
                    continue
            elif unit == "cm":
                if measure < 150 or measure > 193:
                    continue
            else:
                continue

            if not re.match("^#[0-9a-f]{6}$", passport["hcl"]):
                continue

            if passport["ecl"] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
                continue

            if not re.match("^[0-9]{9}$", passport["pid"]):
                continue

            count += 1
        except KeyError:
            pass
        except ValueError:
            pass

    print(count)

part1(passports)
part2(passports)
