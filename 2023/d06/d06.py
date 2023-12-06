import cmath
import math


def get_eq(time, record):
    # record=x(t-x)=-x^2+tx
    res1 = time / 2 - cmath.sqrt(time ** 2 / 4 - record)
    res2 = time / 2 + cmath.sqrt(time ** 2 / 4 - record)
    return int(math.floor(res1.real+1)), int(math.ceil(res2.real-1))


def part1(times, records):
    res=1
    for i in range(len(times)):
        temp = get_eq(times[i],records[i])
        res *=1+temp[1]-temp[0]
    print (res)


def part2(times, records):
    time = int(''.join(str(i) for i in times))
    record = int(''.join(str(i) for i in records))
    part1([time],[record])


filename = "Input06.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
times = list(map(lambda x: int(x), filter("".__ne__, Lines[0].split(":")[1].replace("\n", "").split(" "))))
records = list(map(lambda x: int(x), filter("".__ne__, Lines[1].split(":")[1].replace("\n", "").split(" "))))
part1(times, records)
part2(times, records)
