import regex
import sys

RULE_REGEX = r'^(?P<container>[a-z ]+) bags contain (((?P<contained>\d+ [a-z ]+) bags?(, |.))+|(?:no other bags.))$'

for line in sys.stdin.readlines():
    print(regex.match(RULE_REGEX, line.strip()).groupdict())
