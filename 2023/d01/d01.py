import re




def part1(filename):
    file1 = open(filename, 'r')
    Lines = file1.readlines()
    sum = 0
    for line in Lines:
        nums = [c for c in line if c.isdigit()]
        sum += int(nums[0] + nums[len(nums) - 1])
    print(sum)


def part2(filename):
    file1 = open(filename, 'r')
    Lines = file1.readlines()
    sum = 0
    regex = "(?=(one|two|three|four|five|six|seven|eight|nine))"
    for line in Lines:
        matches = re.findall(regex, line)
        # replace first occurence of number
        if matches:
            match matches[0]:
                case "one":
                    line = line.replace("one", "1one", 1)
                case "two":
                    line = line.replace("two", "2two", 1)
                case "three":
                    line = line.replace("three", "3three", 1)
                case "four":
                    line = line.replace("four", "4four", 1)
                case "five":
                    line = line.replace("five", "5five", 1)
                case "six":
                    line = line.replace("six", "6six", 1)
                case "seven":
                    line = line.replace("seven", "7seven", 1)
                case "eight":
                    line = line.replace("eight", "8eight", 1)
                case "nine":
                    line = line.replace("nine", "9nine", 1)
            # replace all occurences of last written number
            if len(matches) > 1:
                match matches[len(matches) - 1]:
                    case "one":
                        line = line.replace("one", "1")
                    case "two":
                        line = line.replace("two", "2")
                    case "three":
                        line = line.replace("three", "3")
                    case "four":
                        line = line.replace("four", "4")
                    case "five":
                        line = line.replace("five", "5")
                    case "six":
                        line = line.replace("six", "6")
                    case "seven":
                        line = line.replace("seven", "7")
                    case "eight":
                        line = line.replace("eight", "8")
                    case "nine":
                        line = line.replace("nine", "9")

        nums = [c for c in line if c.isdigit()]
        sum += int(nums[0] + nums[len(nums) - 1])
    print(sum)

filename = "Input01.txt"
part1(filename)
part2(filename)