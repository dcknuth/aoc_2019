filename = "input22.txt"
#filename = "test_d22.txt"
#filename = "test_d22-2.txt"
#filename = "test_d22-3.txt"
#filename = "test_d22-4.txt"
N = 10007 # number of cards

with open(filename) as f:
    ls = f.read().strip().split('\n')

cards = [c for c in range(N)]

# part 1
for shuffle in ls:
    if 'new stack' in shuffle:
        cards.reverse()
    elif 'cut ' in shuffle:
        cut_n = shuffle.split()
        cut_n = int(cut_n[1])
        new_cards = cards[cut_n:]
        new_cards.extend(cards[:cut_n])
        cards = new_cards
    elif 'with increment' in shuffle:
        new_cards = [-1] * len(cards)
        inc = shuffle.split()
        inc = int(inc[3])
        i = 0
        while len(cards) > 0:
            c = cards.pop(0)
            new_cards[i] = c
            i = (i + inc) % len(new_cards)
        cards = new_cards
    else:
        print("Unknown shuffle type")

print(cards.index(2019))

# part 2
# first let's print what ended up in index 2020 for part 1
print(f"card in slot 2020 is {cards[2020]}")
# so we will need a way to come up with that given only the index
# TODO
# then we will need to track what lands there over time with the full deck
#  and hope for a repeating value
# TODO
# Then calculate the tail left after a repeat and look up what lands at the index
# TODO

