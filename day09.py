from time_it import time_it
from imac import Imac


testfile = "test09.txt"

def tests():
    with open(testfile) as f:
        programs = f.read().strip().split('\n')

    for i, p in enumerate(programs):
        prog, out_str = p.split()
        plist = [int(x) for x in prog.split(',')]
        cur_mac = Imac(plist, 0, [])
        end_state = cur_mac.run()
        cur_outs = ','.join(cur_mac.outputs)
        if cur_outs == out_str:
            print(f"Program {i} ran correctly")
        else:
            print(f"Program {i} failed")
            print(f"  gave {cur_outs} should have given {out_str}")

tests()

filename = "input09.txt"
with open(filename) as f:
    ls = f.read().strip()

@time_it
def part1(ls):
    plist = [int(x) for x in ls.split(',')]
    cur_mac = Imac(plist, 0, [1])
    end_state = cur_mac.run()
    cur_outs = ','.join(cur_mac.outputs)
    print(f"Part one output {cur_outs}")

part1(ls)

@time_it
def part2(ls):
    plist = [int(x) for x in ls.split(',')]
    cur_mac = Imac(plist, 0, [2])
    end_state = cur_mac.run()
    cur_outs = ','.join(cur_mac.outputs)
    print(f"Part two output {cur_outs}")

part2(ls)
