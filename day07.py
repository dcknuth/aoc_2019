from time_it import time_it
from imac import imac
from itertools import permutations


def amp_circuit(p):
    max_out = -999999
    max_seq = [""]
    for cur_set in permutations(range(0, 5), 5):
        a_out = imac(p, 0, [str(cur_set[0]), str(0)])
        b_out = imac(p, 0, [str(cur_set[1]), a_out[0]])
        c_out = imac(p, 0, [str(cur_set[2]), b_out[0]])
        d_out = imac(p, 0, [str(cur_set[3]), c_out[0]])
        e_out = imac(p, 0, [str(cur_set[4]), d_out[0]])
        if int(e_out[0]) > max_out:
            max_out = int(e_out[0])
            max_seq[0] = f"{cur_set}"
    print(f"{max_out}     {max_seq}")
    return(max_out, max_seq[0])

testfile = "test07.txt"

def tests():
    with open(testfile) as f:
        programs = f.read().strip().split('\n')

    for i, p in enumerate(programs):
        prog, answer = p.split()
        plist = [int(x) for x in prog.split(',')]
        max_out, seq = amp_circuit(plist)
        if max_out == int(answer):
            print(f"Program {i} ran correctly")
            print(f"  with sequence {seq}")
        else:
            print(f"Program {i} failed")
            print(f"  should have given {answer}")

tests()

filename = "input07.txt"
# Read in file. There might be a day or two where this needs to be changed
with open(filename) as f:
    ls = f.read().strip()

@time_it
def part1(ls):
    plist = [int(x) for x in ls.split(',')]
    max_out, seq = amp_circuit(plist)
    print(f"Max is {max_out} with sequence {seq}")

part1(ls)
exit()

@time_it
def part2(ls):
    plist = [int(x) for x in ls[0].split(',')]
    imac(plist, 0)

part2(ls)
