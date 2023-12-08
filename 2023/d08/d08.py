def simulate(start, dirs, nodes, end_cond):
    steps = 0
    while steps == 0 or not end_cond(start):
        for dir in dirs:
            match dir:
                case "L":
                    start = nodes[start][0]
                case "R":
                    start = nodes[start][1]
            steps += 1
    return (start, steps)


def part1(dirs, nodes):
    node = "AAA"
    res = simulate(node, dirs, nodes, lambda x: x == "ZZZ")
    print(res[1])


def gcd(numbers):
    for i in range(0, len(numbers) - 1):
        while numbers[1]:
            numbers[0], numbers[1] = numbers[1], numbers[0] % numbers[1]
        numbers[1] = numbers[i + 1]

    return numbers[0]


def part2(dirs, nodes):
    to_z = [simulate(x, dirs, nodes, lambda x: x[-1] == 'Z') for x in nodes if x[-1] == 'A']
    gcds = gcd([x[1] for x in to_z])
    scm = gcds
    for steps in to_z:
        scm *= steps[1] / gcds
    print(int(scm))


filename = "Input08.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
directions = Lines[0].strip()
nodes = {}
for Line in Lines[2:]:
    split = Line.split("=")
    key = split[0].strip(" \n")
    dirs = split[1].strip("() \n")
    dirs_splitted = dirs.split(", ")
    nodes[key] = (dirs_splitted[0], dirs_splitted[1])
part1(directions, nodes)

part2(directions, nodes)
