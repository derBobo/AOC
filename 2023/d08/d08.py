def part1(dirs, nodes):
    node = "AAA"
    steps = 0
    while node != "ZZZ":
        for dir in dirs:
            match dir:
                case "L":
                    node = nodes[node][0]
                case "R":
                    node = nodes[node][1]
            steps += 1
            if node == "ZZZ":
                break
    print(steps)

def gcd(numbers):
    for i in range(0, len(numbers) - 1):
        while numbers[1]:
            numbers[0], numbers[1] = numbers[1], numbers[0] % numbers[1]
        numbers[1] = numbers[i + 1]

    return numbers[0]

def part2(dirs, nodes):
    start_nodes = [x for x in nodes if x.endswith("A")]
    steps = 0
    steps_to_z =[]
    for node in start_nodes:
        steps = 0
        while not node.endswith("Z"):
            for dir in dirs:
                match dir:
                    case "L":
                        node = nodes[node][0]
                    case "R":
                        node = nodes[node][1]
                steps += 1
        steps_to_z.append(steps)
    gcds= gcd(steps_to_z)
    scm=gcds
    print(steps_to_z)
    for steps in steps_to_z:
        scm*=steps/gcds
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
part1(directions,nodes)

part2(directions, nodes)
