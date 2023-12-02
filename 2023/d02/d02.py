def part1(filename):
    max_rgb = (12, 13, 14)
    file1 = open(filename, 'r')
    Lines = file1.readlines()
    sum = 0
    for line in Lines:
        rgbs = []
        id = 0
        game = line.split(":")
        id = int(game[0].replace("Game ", ""))
        rounds = game[1].split(";")
        for round in rounds:
            rgb = (0, 0, 0)
            colors = round.split(",")
            for color in colors:
                if "red" in color:
                    rgb = (int(color.replace(" red", "")), rgb[1], rgb[2])
                elif "green" in color:
                    rgb = (rgb[0], int(color.replace(" green", "")), rgb[2])
                elif "blue" in color:
                    rgb = (rgb[0], rgb[1], int(color.replace(" blue", "")))
            rgbs.append(rgb)
        possible = True
        for rgb in rgbs:
            if rgb[0] > max_rgb[0] or rgb[1] > max_rgb[1] or rgb[2] > max_rgb[2]:
                possible = False
        if possible:
            sum += id

    print(sum)


def part2(filename):
    file1 = open(filename, 'r')
    Lines = file1.readlines()
    sum = 0
    for line in Lines:
        rgbs = []
        id = 0
        game = line.split(":")
        id = int(game[0].replace("Game ", ""))
        rounds = game[1].split(";")
        for round in rounds:
            rgb = (0, 0, 0)
            colors = round.split(",")
            for color in colors:
                if "red" in color:
                    rgb = (int(color.replace(" red", "")), rgb[1], rgb[2])
                elif "green" in color:
                    rgb = (rgb[0], int(color.replace(" green", "")), rgb[2])
                elif "blue" in color:
                    rgb = (rgb[0], rgb[1], int(color.replace(" blue", "")))
            rgbs.append(rgb)
        min_rgb = (0, 0, 0)
        for rgb in rgbs:
            min_rgb = (max(min_rgb[0],rgb[0]),max(min_rgb[1],rgb[1]),max(min_rgb[2],rgb[2]))
        sum += min_rgb[0]*min_rgb[1]*min_rgb[2]
    print(sum)


filename = "Input02.txt"
part1(filename)
part2(filename)
