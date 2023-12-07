from collections import Counter


def cards_to_index_part1(cards):
    counts = Counter(cards)
    value = 0
    most_common = counts.most_common(1)[0][1]
    if len(counts) > 1:
        second_common = counts.most_common(2)[1][1]
    match most_common:
        case 1:
            value += 10000000000
        case 2:
            value += 20000000000
            if second_common == 2:
                value += 10000000000
        case 3:
            value += 40000000000
            if second_common == 2:
                value += 10000000000
        case 4:
            value += 60000000000
        case 5:
            value += 70000000000
    for i in range(5):
        value += cards[i] * (100 ** (4 - i))
    return value


def part1(hands):
    hands.sort(key=lambda x: cards_to_index_part1(x[0]))
    res = 0
    for i in range(len(hands)):
        res += hands[i][1] * (i + 1)
    print(res)


def cards_to_index_part2(cards):
    cards = [x if x != 11 else 1 for x in cards]

    counts = Counter([x for x in cards if x != 1])
    value = 0
    most_common=cards.count(1)
    if len(counts)>0:
        most_common += counts.most_common(1)[0][1]
        if len(counts) > 1:
            second_common = counts.most_common(2)[1][1]
    match most_common:
        case 1:
            value += 10000000000
            if counts[1] == 1:
                value
        case 2:
            value += 20000000000
            if second_common == 2:
                value += 10000000000
        case 3:
            value += 40000000000
            if second_common == 2:
                value += 10000000000
        case 4:
            value += 60000000000
        case 5:
            value += 70000000000
    for i in range(5):
        value += cards[i] * (100 ** (4 - i))
    return value


def part2(hands):
    hands.sort(key=lambda x: cards_to_index_part2(x[0]))
    res = 0
    for i in range(len(hands)):
        res += hands[i][1] * (i + 1)
    print(res)


def convert_card(x):
    match x:
        case "A":
            return 14
        case "K":
            return 13
        case "Q":
            return 12
        case "J":
            return 11
        case "T":
            return 10
        case _:
            return int(x)


filename = "Input07.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
hands = []
for Line in Lines:
    hand = Line.split(" ")
    bet = int(hand[1])
    cards = list(map(convert_card, hand[0]))
    hands.append((cards, bet))

part1(hands)
part2(hands)
