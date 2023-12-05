import re
import time


def map_to_loc(seed: int, seed_maps):
    for i in range(0, len(seed_maps)):
        for map in seed_maps[i]:
            if seed in map[0]:
                seed = map[1] + seed - map[0].start
                break
    return seed


def map_range(seed_range, seed_maps):
    ranges = [seed_range]
    for i in range(0, len(seed_maps)):
        j = 0
        while j < len(ranges):
            for map in seed_maps[i]:
                if ranges[j].start in map[0]:
                    if ranges[j].stop <= map[0].stop:
                        ranges[j] = range(map[1] + ranges[j].start - map[0].start,
                                          map[1] + ranges[j].stop - map[0].start)
                        break
                    else:
                        ranges.append(range(map[0].stop, ranges[j].stop))
                        ranges[j] = range(map[1] + ranges[j].start - map[0].start,
                                          map[1] + map[0].stop - map[0].start)
                        break
                elif map[0].start < ranges[j].stop <= map[0].stop:
                    ranges.append(range(ranges[j].start, map[0].start))
                    ranges[j] = range(map[1], map[1] + ranges[j].stop - map[0].start)
                    break
                elif map[0].start in ranges[j] and map[0].stop <= ranges[j].stop:
                    ranges.append(range(ranges[j].start, map[0].start))
                    ranges.append(range(map[0].stop, ranges[j].stop))
                    ranges[j] = range(map[1], map[1] + map[0].stop - map[0].start)
                    break

            j += 1
    return ranges


def part1(seeds, seed_maps):
    res = min(list(map(lambda x: map_to_loc(x, seed_maps), seeds)))
    print(res)


def part2(seeds, seed_maps):
    seeds_range = []
    for i in range(0, len(seeds), 2):
        seeds_range.append(min(map(lambda x: x.start, map_range(range(seeds[i], seeds[i] + seeds[i + 1]), seed_maps))))
    res = min(seeds_range)
    print(res)

filename = "Input05.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
seeds = []
seed_maps = []
i = 0
while i < len(Lines):
    if "seeds:" in Lines[i]:
        seeds = list(map(lambda x: int(x), filter("".__ne__, Lines[i].split(":")[1].replace("\n", "").split(" "))))
        i += 1
    elif re.match(r'^\d', Lines[i]):
        ranges = []
        while i < len(Lines) and not "\n" == Lines[i]:
            splitted = list(map(lambda x: int(x), Lines[i].split(" ")))
            ranges.append((range(splitted[1], splitted[1] + splitted[2]), splitted[0]))
            i += 1
        seed_maps.append(ranges)
    i += 1
part1(seeds, seed_maps)
part2(seeds, seed_maps)
