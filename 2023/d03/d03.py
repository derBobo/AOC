def part1(symbols):
    sum = 0
    for x in range(0, len(symbols)):
        for y in range(0, len(symbols[x])):
            if symbols[x][y][0] and not symbols[x][y - 1][0]:
                number = int(symbols[x][y][1])
                while symbols[x][y + 1][0]:
                    number = number * 10 + int(symbols[x][y + 1][1])
                    y += 1
                sum += number

    print(sum)


def part2(symbols):
    sum = 0
    for x in range(0, len(symbols)):
        for y in range(0, len(symbols[x])):
            if symbols[x][y][1] == "*":
                gears = []
                for deltax in [-1, 0, 1]:
                    for deltay in [-1, 0, 1]:
                        if symbols[x + deltax][y + deltay][0]:
                            while symbols[x + deltax][y + deltay - 1][0]:
                                deltay -= 1
                            if (x + deltax, y + deltay) not in gears:
                                gears.append((x + deltax, y + deltay))
                if len(gears) == 2:
                    x1 = gears[0][0]
                    y1 = gears[0][1]
                    num1 = int(symbols[x1][y1][1])
                    while symbols[x1][y1 + 1][0]:
                        num1 = num1 * 10 + int(symbols[x1][y1 + 1][1])
                        y1 += 1
                    x2 = gears[1][0]
                    y2 = gears[1][1]
                    num2 = int(symbols[x2][y2][1])
                    while symbols[x2][y2 + 1][0]:
                        num2 = num2 * 10 + int(symbols[x2][y2 + 1][1])
                        y2 += 1
                    sum += num1*num2
    print(sum)


filename = "Input03.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
symbols = [[(False, ".")] * (len(Lines[0].replace("\n", "")) + 2)]
for line in Lines:
    line = line.replace("\n", "")
    temp = [(False, ".")]
    temp.extend(map(lambda x: (False, x), line))
    temp.append((False, "."))
    symbols.append(temp)
symbols.append([(False, ".")] * (len(Lines[0].replace("\n", "")) + 2))
for x in range(0, len(symbols)):
    for y in range(0, len(symbols[x])):
        if not symbols[x][y][1].isdigit() and not symbols[x][y][1] == ".":
            for deltax in [-1, 0, 1]:
                for deltay in [-1, 0, 1]:
                    if symbols[x + deltax][y + deltay][1].isdigit() and not symbols[x + deltax][y + deltay][0]:
                        y1 = y + deltay
                        while symbols[x + deltax][y1][1].isdigit():
                            symbols[x + deltax][y1] = (True, symbols[x + deltax][y1][1])
                            y1 += 1
                        y1 = y + deltay
                        while symbols[x + deltax][y1][1].isdigit():
                            symbols[x + deltax][y1] = (True, symbols[x + deltax][y1][1])
                            y1 -= 1
part1(symbols)
part2(symbols)
