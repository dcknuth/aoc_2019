from time_it import time_it
from imac import imac

testfile = "test05.txt"
resultfile = "test_results05.txt"

def tests():
    with open(testfile) as f:
        programs = f.read().strip().split('\n')

    with open(resultfile) as f:
        results = f.read().strip().split('\n')

    for i, p in enumerate(programs):
        plist = [int(x) for x in p.split(',')]
        rlist = [int(x) for x in results[i].split(',')]
        imac(plist, 0)
        if plist == rlist:
            print(f"Program {i} ran correctly")
            print(f"Value at 0 is {plist[0]}")
        else:
            print(f"Program {i} failed")
            print(f"P: {plist}\nR: {rlist}")

#tests()
#exit()

filename = "input05.txt"

# Read in file. There might be a day or two where this needs to be changed
with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    plist = [int(x) for x in ls[0].split(',')]
    imac(plist, 0)

# have to do this to time OK with an input
import builtins
def my_input(prompt=""):
    return("1")

builtins.input = my_input
part1(ls)


@time_it
def part2(ls):
    plist = [int(x) for x in ls[0].split(',')]
    imac(plist, 0)


import builtins
def my_input(prompt=""):
    return("5")

builtins.input = my_input
part2(ls)
