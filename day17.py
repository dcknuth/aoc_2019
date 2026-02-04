from imac import Imac

filename = "input17.txt"

with open(filename) as f:
    ls = f.read().strip().split(',')

p = [int(x) for x in ls]
vm = Imac(p, 0, [])
state = vm.run()
m = []
cur_row = []
bot_info = ['!', '-1', '-1'] # direction y x
y = 0
x = 0
for i, c in enumerate(vm.outputs):
    if c == '10':
        m.append(cur_row)
        cur_row = []
        y += 1
        continue
    l = chr(int(c))
    if l == 'X':
        cur_row.append('.')
        bot_info[0] = l
        bot_info[1] = y
        bot_info[2] = x
        x += 1
    elif l == '^' or l == 'v' or l == '<' or l == '>':
        cur_row.append('#')
        bot_info[0] = l
        bot_info[1] = y
        bot_info[2] = x
        x += 1
    else:
        cur_row.append(l)
        x += 1
# There seems to be a last empty row, pop that
m.pop()

def printm(m):
    for row in m:
        for c in row:
            print(c, end='')
        print()

printm(m)

dirs = [(1, 0), (-1, 0), (0, -1), (0, 1)]
intersections = []
total = 0
num_scaffold = 0
for y, row in enumerate(m):
    for x, c in enumerate(row):
        if c == '#':
            i = 0
            num_scaffold += 1
            for dy, dx in dirs:
                ty = y+dy
                tx = x+dx
                if ty > -1 and ty < len(m) and tx > -1 and tx < len(row):
                    if m[ty][tx] == '#':
                        i += 1
            if i > 2:
                intersections.append((y, x))
                total += y * x

print(total)
print(num_scaffold)

# Part 2
# Approach: Manually enter the programs
main_p = ['A', 'B', 'A', 'C', 'A', 'B', 'C', 'A', 'B', 'C']
a = ['R', '8', 'R', '10', 'R', '10']
b = ['R', '4', 'R', '8', 'R', '10', 'R', '12']
c = ['R', '12', 'R', '4', 'L', '12', 'L', '12']
a_string = ','.join(a)
ap = [ord(x) for x in a_string]
ap.append(10)
b_string = ','.join(b)
bp = [ord(x) for x in b_string]
bp.append(10)
c_string = ','.join(c)
cp = [ord(x) for x in c_string]
cp.append(10)
mp = ','.join(main_p)
mp = [ord(x) for x in mp]
mp.append(10)

print(f"Trying\n{mp=}")
print(f"{a=}")
print(ap)
print(f"{b=}")
print(bp)
print(f"{c=}")
print(cp)

p = [int(x) for x in ls]
p[0] = 2
vm2 = Imac(p, 0, [])
state = vm2.run()
# should prompt for main prog
vm2.add_input(mp)
state = vm2.run()
# should prompt for A prog
vm2.add_input(ap)
state = vm2.run()
# should prompt for B prog
vm2.add_input(bp)
state = vm2.run()
# should prompt for C prog
vm2.add_input(cp)
state = vm2.run()
# should prompt for video feed
vm2.add_input([ord('n'), 10])
state = vm2.run()
# output should be the amount of space dust
outputs = vm2.get_output()
print(outputs[-1])
