from imac import Imac
import networkx as nx
from collections import Counter
from copy import deepcopy

filename = "input15.txt"

with open(filename) as f:
    ls = f.read().strip().split(',')

p = [int(x) for x in ls]

# assume we start at 49, 49 on the grid
# assume 0-99 matrix(room) for printing purposes
m = [[' ' for x in range(100)] for y in range(100)]
cur_pos = [49, 49]
m[cur_pos[0]][cur_pos[1]] = '.'
ox_pos = [-1, -1]
g = nx.Graph()

def print_m(m, cur_pos):
    under_droid = m[cur_pos[0]][cur_pos[1]]
    m[cur_pos[0]][cur_pos[1]] = 'D'
    for y in range(99, -1, -1):
        for x in range(100):
            print(m[y][x], end='')
        print()
    m[cur_pos[0]][cur_pos[1]] = under_droid

directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]
turn_left = {(0, -1):(-1, 0),
              (-1, 0):(0, 1),
              (0, 1):(1, 0),
              (1, 0): (0, -1)}
turn_right = {(0, -1):(1, 0),
              (1, 0):(0, 1),
              (0, 1):(-1, 0),
              (-1, 0): (0, -1)}
get_command = {(1, 0):1, (-1, 0):2, (0, -1):3, (0, 1):4}

def cm(vm, try_dir, cur_pos, ox_pos, m, g):
    '''Try the direction and return True if we moved and False otherwise'''
    ty = cur_pos[0] + try_dir[0]
    tx = cur_pos[1] + try_dir[1]
    cm = get_command[try_dir]
    vm.add_input([cm])
    cur_state = vm.run()
    outputs = vm.get_output()
    if len(outputs) != 1:
        print("Outputs should always have one item")
        exit(1)
    status = outputs.pop()
    if status == '0':
        # hit a wall
        m[ty][tx] = '#'
        return(False)
    elif status == '1':
        # entered blank area
        m[ty][tx] = '.'
        g.add_edge((ty, tx), tuple(cur_pos))
        cur_pos[0] += try_dir[0]
        cur_pos[1] += try_dir[1]
        return(True)
    elif status == '2':
        # found oxygen and moved onto that area
        ox_pos[0] = ty
        ox_pos[1] = tx
        m[ty][tx] = 'O'
        g.add_edge((ty, tx), tuple(cur_pos))
        cur_pos[0] += try_dir[0]
        cur_pos[1] += try_dir[1]
        return(True)
    else:
        print("Unknown status code")
        exit(1)

vm = Imac(p, 0, [])
num_moves = 0
visited = Counter()
visited[tuple(cur_pos)] += 1
my_dir = (0, -1) # west
done_exploring = False

# APPROACH: make deepcopies of vms and try each direction from the current.
#  Each successful move is pushed onto a stack of future current locations
#  We should run out of unvisited locations that are valid to move to and
#  have a full network

moves = []
moves.append((vm, cur_pos))
while len(moves) > 0:
    if num_moves % 10 == 0:
        print_m(m, cur_pos)
    new_moves = []
    for vm, cur_pos in moves:
        # try a move in each direction
        for try_dir in directions:
            ty = cur_pos[0] + try_dir[0]
            tx = cur_pos[1] + try_dir[1]
            if m[ty][tx] == ' ':
                cur_vm = deepcopy(vm)
                try_pos = cur_pos.copy()
                moved = cm(cur_vm, try_dir, try_pos, ox_pos, m, g)
                if moved:
                    # push to moves
                    new_moves.append((cur_vm, try_pos))
                    visited[tuple(try_pos)] += 1
    moves = new_moves
    num_moves += 1

moves = nx.shortest_path_length(g, source=(49,49), target=tuple(ox_pos))
print(f"Moves to Ox tank is {moves}")
lengths = nx.single_source_shortest_path_length(g, source=tuple(ox_pos))
lengths = list(lengths.values())
lengths.sort(reverse=True)
print(f"Ox everywhere in {lengths[0]} minutes")
