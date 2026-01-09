from collections import defaultdict, Counter

#filename = "test14.txt"
#filename = "test14-2.txt"
#filename = "test14-3.txt"
#filename = "test14-4.txt"
#filename = "test14-5.txt"
filename = "input14.txt"

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
# add an identity case for ORE
outputs['ORE'] = {'ORE':1}

def expand_level(items, remainders):
    '''Items dict and remainders of items
    We expand breadth-first a set of items at a time
    Return the amounts of everything needed as a dict including remainders
    so it works for part 2'''
    new_items = Counter()
    for item in items.keys():
        if item == 'ORE':
            new_items['ORE'] += items['ORE']
        else:
            if remainders[item] >= items[item]:
                remainders[item] -= items[item]
                continue
            item_need = items[item] - remainders[item]
            remainders[item] = 0
            production_step = outputs[item][item]
            pairs = ((k, v) for k, v in outputs[item].items() if k != item)
            steps = item_need // production_step
            if steps * production_step < item_need:
                steps += 1
            for sub_item, sub_need in pairs:
                new_need = steps * sub_need
                new_need -= remainders[sub_need]
                new_items[sub_item] += new_need
            remainders[item] += steps * production_step - item_need
    return(new_items, remainders)

items = Counter({'FUEL':1})
remainders = Counter()
while True:
    items, remainders = expand_level(items, remainders)
    if len(items) == 1 and next(iter(items)) == 'ORE':
        break
print(f"Part one is {items['ORE']}")

fuel = 1
while items['ORE'] < 1000000000000:
    items['FUEL'] = 1
    while True:
        items, remainders = expand_level(items, remainders)
        if len(items) == 1 and next(iter(items)) == 'ORE':
            break
    if items['ORE'] < 1000000000000:
        fuel += 1
    # if fuel % 10000 == 0:
    #     print(fuel)
print(f"Part two, max fuel is {fuel}")
