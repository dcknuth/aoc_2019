from collections import defaultdict, Counter
from functools import cache

filename = "test14.txt"

with open(filename) as f:
    ls = f.read().strip().split('\n')

# will have the output name as the key and the output amount in the dict
#  along with all the inputs
outputs = defaultdict(dict)
for l in ls:
    inputs, outs = l.split(' => ')
    if ',' in inputs:
        items = list(inputs.split(', '))
    else:
        items = [inputs]
    out_q, out_name = outs.split()
    out_q = int(out_q)
    outputs[out_name][out_name] = out_q
    for i in items:
        q, name = i.split()
        q = int(q)
        outputs[out_name][name] = q

@cache
def ore_equiv(name, n):
    ''' Name of the item you want and n is the quantity needed.
    Returns the amount of ORE needed and a Counter of the left over items'''
    inputs = list(outputs[name].keys())
    production_step = outputs[name][name]
    inputs.remove(name)
    if len(inputs) == 1 and inputs[0] == 'ORE':
        # TODO calc production steps needed and remainder
        steps = n // production_step
        if steps * production_step == n:
            needed = outputs[name][inputs[0]]
        return(n*needed, d)
    if len(inputs) == 1:
        return(ore_equiv(inputs[0], n*outputs[name][inputs[0]]))
    # there is more than one input
    in_pairs = ((k, v) for k, v in outputs[name].items() if k != name)
    # TODO need a way to collect both values vvvv
    return(n*sum(map(ore_equiv, in_pairs)), d)

ans_p1 = ore_equiv('FUEL', 1)
print(f"Part one is {ans_p1}")
