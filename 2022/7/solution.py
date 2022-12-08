import sys

root = {
        "parent": None,
        "name": "/",
        "children": {},
        "files": {},
        "size": 0
}

current = root

for line in sys.stdin.readlines():
    line = line.strip().split()
    if line[0] == "$":
        cmd = line[1]
        if cmd == "ls":
            continue

        arg = line[2]
        if arg == "..":
            current = current["parent"]
        elif arg == "/":
            continue
        else:
            current = current["children"][arg]
    else:
        size_or_type, name = line[0:]
        if size_or_type == "dir":
            current["children"][name] = {
                    "parent": current,
                    "name": name,
                    "children": {},
                    "files": {}
            }


        else:
            current["files"][name] = int(size_or_type)

pt1 = 0
alldirs = []

def size(fs):
    global pt1
    global alldirs

    children = [size(child) for child in fs["children"].values()]
    files = sum(fs["files"].values())

    me = sum(children) + files

    if me <= 100000:
        print(f"Adding {fs['name']} at size {me}")
        pt1 += me
    else:
        print(f"Not adding {fs['name']} at size {me}")

    alldirs.append(me) 
    return me


used = size(root)
print(f"Part 1: {pt1}")

total = 70000000
needed = 30000000
free = total - used
to_reclaim = needed - free

alldirs.sort()

for d in alldirs: 
    if d > to_reclaim:
        print(f"Part 2: {d}")
        break

