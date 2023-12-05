def part1(cards):
    sum=0
    for card in cards:
        numbers_won = [value for value in card[0] if value in card[1]]
        if numbers_won:
            sum+= 2**(len(numbers_won)-1)
    print(sum)
def part2(cards):
    sum = 0
    for i in range(0,len(cards)):
        numbers_won = [value for value in cards[i][0] if value in cards[i][1]]
        if numbers_won:
            for j in range(i+1,min(i+len(numbers_won)+1,len(cards))):
                cards[j]= (cards[j][0],cards[j][1],cards[j][2] +cards[i][2])
    for card in cards:
        sum+= card[2]
    print(sum)

filename = "Input04.txt"
file1 = open(filename, 'r')
Lines = file1.readlines()
cards= []
for line in Lines:
    line = line.replace("\n", "")
    n_only = line.split(":")[1]
    n_splitted =n_only.split("|")
    winning_numbers = list(map(lambda x: int(x),filter("".__ne__,n_splitted[0].split(" "))))
    my_numbers = list(map(lambda x: int(x),filter("".__ne__,n_splitted[1].split(" "))))
    cards.append((winning_numbers,my_numbers,1))
part1(cards)
part2(cards)
