from imac import imac

testfile = "test02.txt"
resultfile = "test_results02.txt"


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


inputfile = "input02.txt"
def part1():
    with open(inputfile) as f:
        p = [int(x) for x in f.read().strip().split(',')]
    
    # Replace values as indicated in the puzzle
    p[1] = 12
    p[2] = 2

    imac(p, 0)
    print(f"Value at 0 is {p[0]}")

part1()

from itertools import product
def part2():
    with open(inputfile) as f:
        p = [int(x) for x in f.read().strip().split(',')]
    
    for (noun, verb) in product(range(0, 100), range(0, 100)):
        plist = p.copy()
        plist[1] = noun
        plist[2] = verb
        imac(plist, 0)
        if plist[0] == 19690720:
            print(f"Part 2 answer is {100 * noun + verb}")
            break

part2()
