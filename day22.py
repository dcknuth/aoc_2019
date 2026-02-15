filename = "input22.txt"
#filename = "test_d22.txt"
#filename = "test_d22-2.txt"
#filename = "test_d22-3.txt"
#filename = "test_d22-4.txt"
#filename = "test_d22-5.txt"
#filename = "test_d22-6.txt"
#filename = "test_d22-7.txt"
#filename = "test_d22-9.txt"
N = 10007 # number of cards
#N = 10

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
# Read the readme for this day to see where this comes from
def shuffle(i, N, ls):
    a, b = 1, 0
    for s in ls:
        if 'new stack' in s:
            i = N - 1 - i
            a *= -1
            b = N - 1 - b
        elif 'cut ' in s:
            cut_n = s.split()
            cut_n = int(cut_n[1])
            i = (i - cut_n) % N
            b -= cut_n
        elif 'with increment' in s:
            inc = s.split()
            inc = int(inc[3])
            i = (i * inc) % N
            a *= inc
            b *= inc
        else:
            print("Unknown shuffle type")
    return(i, a % N, b % N)

# redo part 1 to make sure this is still working for that
card, a, b = shuffle(2019, N, ls)
print(card)

# change for part 2
N = 119315717514047
times = 101741582076661

def big_shuffle(position_needed, N, times, ls):
    _, c, d = shuffle(position_needed, N, ls)
    a, b = pow(c, -1, N), (-d * pow(c, -1, N)) % N
    p1 = pow(a, times, N)
    p2 = (p1 - 1) * pow(a - 1, N - 2, N)
    return (( p1 * position_needed ) + ( b * p2 )) % N

print(big_shuffle(2020, N, times, ls))
