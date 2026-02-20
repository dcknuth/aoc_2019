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
