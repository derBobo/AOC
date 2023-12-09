def extrapolate_last(sequence):
    if all([x == 0 for x in sequence]):
        return 0
    else:
        diffs = [sequence[i + 1] - sequence[i] for i in range(len(sequence) - 1)]
        return sequence[-1] + extrapolate_last(diffs)


def extrapolate_first(sequence):
    if all([x == 0 for x in sequence]):
        return 0
    else:
        diffs = [sequence[i + 1] - sequence[i] for i in range(len(sequence) - 1)]
        return sequence[0] - extrapolate_first(diffs)


def part1(sequences):
    extrapolated = [extrapolate_last(seq) for seq in sequences]
    res = sum(extrapolated)
    print(res)


def part2(sequences):
    extrapolated = [extrapolate_first(seq) for seq in sequences]
    res = sum(extrapolated)
    print(res)


filename = "Input09.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
sequences = []
for Line in Lines:
    sequence = [int(x) for x in Line.split(" ")]
    sequences.append(sequence)
part1(sequences)
part2(sequences)
