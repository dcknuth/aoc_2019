from imac import Imac

filename = "input21.txt"

with open(filename) as f:
    ls = f.read().strip().split(',')

p = [int(i) for i in ls]
vm = Imac(p, 0, [])

# Part 1
# get the prompt
vm.run()
cur_out = vm.get_output()
txt_out = ''.join([chr(int(c)) for c in cur_out])
print(txt_out)

# try example springscript program
ssp_txt = """NOT A J
NOT B T
AND T J
NOT C T
AND T J
AND D J""".split('\n')
ssp = []
for s in ssp_txt:
    ssp.extend([ord(c) for c in s])
    ssp.append(10)
ssp.extend([ord(c) for c in "WALK"])
ssp.append(10)
vm.add_input(ssp)
vm.run()
cur_out = vm.get_output()
txt_out = ''.join([chr(int(c)) for c in cur_out])
print(txt_out)

# jump if we are coming to a hole and the landing spot is safe
vm = Imac(p, 0, [])
vm.run()
cur_out = vm.get_output()
txt_out = ''.join([chr(int(c)) for c in cur_out])
ssp_txt = """NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J""".split('\n')
ssp = []
for s in ssp_txt:
    ssp.extend([ord(c) for c in s])
    ssp.append(10)
ssp.extend([ord(c) for c in "WALK"])
ssp.append(10)
vm.add_input(ssp)
vm.run()
cur_out = vm.get_output()
txt_out = []
for s in cur_out:
    if len(s) < 4:
        txt_out.append(chr(int(s)))
    else:
        txt_out.append(s)
print(''.join(txt_out))

# Part 2
# If we try the part 1 program, it will jump too soon in some configurations
# jump if we are coming to a hole and the landing spot is safe
vm = Imac(p, 0, [])
vm.run()
cur_out = vm.get_output()
txt_out = ''.join([chr(int(c)) for c in cur_out])
ssp_txt = """NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J""".split('\n')
ssp = []
for s in ssp_txt:
    ssp.extend([ord(c) for c in s])
    ssp.append(10)
ssp.extend([ord(c) for c in "RUN"])
ssp.append(10)
vm.add_input(ssp)
vm.run()
cur_out = vm.get_output()
txt_out = []
for s in cur_out:
    if len(s) < 4:
        txt_out.append(chr(int(s)))
    else:
        txt_out.append(s)
print(''.join(txt_out))