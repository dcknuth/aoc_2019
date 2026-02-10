from imac import Imac
from copy import deepcopy

filename = "input19.txt"

with open(filename) as f:
    ls = f.read().strip().split(',')
p = [int(x) for x in ls]

total = 0
m = []
vm = Imac(p, 0, [])
for y in range(50):
    row = []
    for x in range(50):
        cur_vm = deepcopy(vm)
        cur_vm.add_input([x, y])
        status = cur_vm.run()
        pulled = cur_vm.get_output()
        if len(pulled) > 0 and pulled[0] == '1':
                total += int(pulled[0])
                row.append('#')
        else:
             row.append('.')
    m.append(row)

print(total)
for row in m:
    print(''.join(row))

# part 2
# Eyeballing from part 1
# Then try to capture the first 100x100 square in a reasonable loop
widths = dict() # y as the key and the first and last x beam position
y = 1500
found = False
while not found:
    row = []
    x_start = 1650
    for x in range(x_start, x_start + 800):
        cur_vm = deepcopy(vm)
        cur_vm.add_input([x, y])
        status = cur_vm.run()
        pulled = cur_vm.get_output()
        if len(pulled) > 0 and pulled[0] == '1':
                row.append('#')
        else:
             row.append('.')
    cur_beam = ''.join(row)
    last_beam_x = cur_beam.rindex('#') + x_start
    first_beam_x = cur_beam.index('#') + x_start
    widths[y] = [first_beam_x, last_beam_x]
    if y-99 in widths:
        if widths[y-99][1] - widths[y][0] >= 99:
            found = True
            print(f"{x=} {y=} {widths[y-99]} {widths[y]}")
            print(f"Upper-left of square at x={widths[y-99][1]-99} y={y-99}")
            print(f"Part 2 answer is {(widths[y-99][1]-99) * 10000 + (y-99)}")
    y += 1



