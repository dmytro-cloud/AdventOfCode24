import itertools

fileContents = open("input1.txt")
arr = fileContents.read().split("\n")


increasing = [1, 2, 3]
decreasing = [-1, -2, -3]


def is_safe(levels):
    if levels[0] == levels[1]:
        return False
    not_increasing = False
    not_decreasing = False
    for i, level in enumerate(levels[1:]):
        if levels[i] - level not in increasing:
            # print("not_increasing", levels[i] - level)
            not_increasing = True
        if levels[i] - level not in decreasing:
            # print("not_decreasing", levels[i] - level)
            not_decreasing = True
    if not_increasing and not_decreasing:
        return False
    else:
        return True


safe = 0
for line in arr:
    levels = line.split(" ")
    levels = [int(i) for i in levels]
    if is_safe(levels):
        print(*levels)
        safe += 1
    else:
        level_subsets = itertools.combinations(levels, len(levels) - 1)
        found_safe = False
        for sub in level_subsets:
            if is_safe(sub):
                found_safe = True
        if found_safe:
            print(*levels)
            safe += 1

print(safe)