from time_it import time_it
from imac import Imac
from itertools import permutations


def amp_circuit(p):
    max_out = -999999
    max_seq = [""]
    for cur_set in permutations(range(5, 10), 5):
        amp_a = Imac(p, 0, [str(cur_set[0])])
        amp_b = Imac(p, 0, [str(cur_set[1])])
        amp_c = Imac(p, 0, [str(cur_set[2])])
        amp_d = Imac(p, 0, [str(cur_set[3])])
        amp_e = Imac(p, 0, [str(cur_set[4])])
        
        stop_circuit = False
        out_str = ["0"]
        while not stop_circuit:
            if len(out_str) > 0:
                signal_input = out_str.pop()
            status = amp_a.run([signal_input], out_str)
            if status == -1:
                stop_circuit = True
                if len(out_str) > 0:
                    signal_input = out_str.pop()
            else:
                signal_input = out_str.pop()
            status = amp_b.run([signal_input], out_str)
            if status == -1:
                stop_circuit = True
                if len(out_str) > 0:
                    signal_input = out_str.pop()
            else:
                signal_input = out_str.pop()
            status = amp_c.run([signal_input], out_str)
            if status == -1:
                stop_circuit = True
                if len(out_str) > 0:
                    signal_input = out_str.pop()
            else:
                signal_input = out_str.pop()
            status = amp_d.run([signal_input], out_str)
            if status == -1:
                stop_circuit = True
                if len(out_str) > 0:
                    signal_input = out_str.pop()
            else:
                signal_input = out_str.pop()
            status = amp_e.run([signal_input], out_str)
            if status == -1:
                stop_circuit = True
                if len(out_str) > 0:
                    if int(out_str[0]) > max_out:
                        final_out = int(out_str[0])
        if len(out_str) > 0:
            final_out = out_str.pop()
        if int(final_out) > max_out:
            max_out = int(final_out)
            max_seq[0] = f"{cur_set}"
    print(f"{max_out}     {max_seq}")
    return(max_out, max_seq[0])

testfile = "test07-2.txt"

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
with open(filename) as f:
    ls = f.read().strip()

@time_it
def part2(ls):
    plist = [int(x) for x in ls.split(',')]
    max_out, seq = amp_circuit(plist)
    print(f"Max of {max_out} with {seq}")

part2(ls)
