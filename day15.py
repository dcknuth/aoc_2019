from imac import Imac
import networkx as nx
from collections import Counter

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

# APPROACH: make sure we know adjacent info.
#  Prefer unvisited spots. If there are unvisited options:
#  Turn right, then straight, then left

# start with just moving a given number of times
while num_moves < 100000 and not done_exploring:
    # if num_moves % 200 == 0:
    #     print_m(m, cur_pos)
    
    # make sure we know what is around this location
    for try_dir in directions:
        ty = cur_pos[0] + try_dir[0]
        tx = cur_pos[1] + try_dir[1]
        if m[ty][tx] == ' ':
            moved = cm(vm, try_dir, cur_pos, ox_pos, m, g)
            if moved:
                # move back
                back_y = try_dir[0] * -1
                back_x = try_dir[1] * -1
                cm(vm, (back_y, back_x), cur_pos, ox_pos, m, g)
    
    # try unvisited locations, right-most preference
    my_dir = turn_right[my_dir]
    have_dir = False
    visit_count = []
    for i in range(4):
        ty = cur_pos[0] + my_dir[0]
        tx = cur_pos[1] + my_dir[1]
        if (ty, tx) not in visited and (m[ty][tx] == '.' or m[ty][tx] == 'O'):
            have_dir = True
            break
        elif m[ty][tx] == '.' or m[ty][tx] == 'O':
            visit_count.append((visited[(ty, tx)], my_dir))
        my_dir = turn_left[my_dir]
    if have_dir:
        moved = cm(vm, my_dir, cur_pos, ox_pos, m, g)
        if not moved:
            print("Error: we should be able to move to the selected direction")
            exit(1)
        visited[tuple(cur_pos)] += 1
    else:
        # least visit count with right preference
        visit_count.sort()
        if visit_count[0][0] == 4:
            done_exploring = True
            print("We visited someplace 4 times")
        my_dir = visit_count[0][1]
        moved = cm(vm, my_dir, cur_pos, ox_pos, m, g)
        if not moved:
            print("Error: we should be able to move to the selected direction")
            exit(1)
        visited[tuple(cur_pos)] += 1

print(f"Ox tank at {ox_pos}")
moves = nx.shortest_path_length(g, source=(49,49), target=tuple(ox_pos))
print(f"Moves to Ox tank is {moves}")
