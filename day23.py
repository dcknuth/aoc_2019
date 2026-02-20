from imac import Imac
from collections import defaultdict

filename = "input23.txt"

with open(filename) as f:
    ls = f.read().strip().split(',')

p = [int(i) for i in ls]
network = [Imac(p, 0, [i]) for i in range(50)]

# boot each and give network address, the i above in input
for vm in network:
    vm.run()

# loop until we have a packet to 255
done = False
while not done:
    # input destinations not 0-49
    inputs_by_addr = defaultdict(list)
    # inputs for our 50 machines
    inputs = [[] for x in range(len(network))]
    for i in range(len(network)):
        # get outputs
        cur_out = network[i].get_output()
        while len(cur_out) > 0:
            addr = int(cur_out.pop(0))
            x = int(cur_out.pop(0))
            y = int(cur_out.pop(0))
            if addr < 50:
                inputs[addr].extend([x, y])
            elif addr == 255:
                done = True
                inputs_by_addr[255].extend([x, y])
            else:
                inputs_by_addr[255].extend([x, y])
    # handle inputs
    for i in range(len(network)):
        if len(inputs[i]) > 0:
            network[i].add_input(inputs[i])
        network[i].add_input([-1])
    # run systems again if not finished
    if len(inputs_by_addr[255]) < 1:
        for vm in network:
            vm.run()

print(inputs_by_addr[255])

# Part 2: need the first Y value delivered from 255 twice in a row
# To start, let just print status every time something goes to 255
done = False
times = 0
old_NAT = inputs_by_addr[255].copy()
# we need to transfer the x, y from part 1 to vm 0
network[0].add_input(inputs_by_addr[255])
# then do a run to get the network going again
for vm in network:
    vm.run
# and restart our modified loop
while not done and times < 1000:
    times += 1
    inputs_by_addr = defaultdict(list)
    # inputs for our 50 machines
    inputs = [[] for x in range(len(network))]
    found_incoming = False
    for i in range(len(network)):
        # get outputs
        cur_out = network[i].get_output()
        while len(cur_out) > 0:
            addr = int(cur_out.pop(0))
            x = int(cur_out.pop(0))
            y = int(cur_out.pop(0))
            if addr < 50:
                inputs[addr].extend([x, y])
            elif addr == 255:
                #print(f"New 255 packet is {x=}, {y=}")
                inputs_by_addr[255] = [x, y]
            else:
                print("We should not have a non-255, non-0-49 addr")
    # handle inputs
    for i in range(len(network)):
        if len(inputs[i]) > 0:
            found_incoming = True
            network[i].add_input(inputs[i])
        network[i].add_input([-1])
    if not found_incoming:
        # all is quiet, send 255 state to 0
        #print(f"sending to 0 {inputs_by_addr[255]}")
        network[0].add_input(inputs_by_addr[255])
        #print(f"old x y {old_NAT}")
        if len(inputs_by_addr[255]) > 0:
            if old_NAT[1] == inputs_by_addr[255][1]:
                done = True
            old_NAT = inputs_by_addr[255].copy()
    # run systems again
    if len(inputs_by_addr[255]) < 1:
        for vm in network:
            vm.run()

print(old_NAT[1])