from time_it import time_it
from imac import Imac


filename = "input13.txt"
with open(filename) as f:
    ls = f.read().strip()


def process_output(op, old_status):
    screen = dict()
    score = -1
    ball_pos = [-1, -1]
    ball_dir = [0, 0]
    paddle_pos = [-1, -1]
    for i in range(0, len(op), 3):
        x = int(op[i])
        y = int(op[i+1])
        tile_id = op[i+2]
        if x < 0:
            score = tile_id
        else:
            screen[(x, y)] = tile_id
            if tile_id == '4':
                ball_pos = [x, y]
                if old_status[0] > -1:
                    ball_dir[0] = ball_pos[0] - old_status[1][0]
                    ball_dir[1] = ball_pos[1] - old_status[1][1]
            elif tile_id == '3':
                paddle_pos[0] = x
                paddle_pos[1] = y

def print_screen(outputs):
    screen = {}
    for i in range(0, len(outputs), 3):
        x = int(outputs[i])
        y = int(outputs[i+1])
        tile_id = outputs[i+2]
        screen[(x,y)] = tile_id
    max_x = max([k[0] for k in screen.keys()])
    max_y = max([k[1] for k in screen.keys()])
    for y in range(max_y+1):
        row = ''
        for x in range(max_x+1):
            tile_id = screen.get((x,y), '0')
            if tile_id == '0':
                row += ' '  # empty
            elif tile_id == '1':
                row += '#'  # wall
            elif tile_id == '2':
                row += 'B'  # block
            elif tile_id == '3':
                row += '-'  # paddle
            elif tile_id == '4':
                row += 'O'  # ball
        print(row)

@time_it
def part1(ls):
    plist = [int(x) for x in ls.split(',')]
    cur_mac = Imac(plist, 0, [])
    end_state = cur_mac.run()
    block_tile_count = 0
    for i, cur_block in enumerate(cur_mac.outputs):
        if (i+1)%3 == 0 and cur_block == '2':
            block_tile_count += 1
    print(f"Part one block count is: {block_tile_count}")
    #print("Final screen:")
    #print_screen(cur_mac.outputs)

part1(ls)

@time_it
def part2(ls):
    plist = [int(x) for x in ls.split(',')]
    cur_mac = Imac(plist, 0, [])
    cur_mac.pg[0] = 2 # set to play for free
    mac_state = 0 # waiting to run
    score = 0
    while mac_state != -1:
        mac_state = cur_mac.run()
        # should have a screen of output and be waiting for input
        # TODO drain output, look for the ball
        cur_out = cur_mac.get_output()
        cur_status = process_output(cur_out)
        # TODO update direction of travel, update the score
        # TODO provide joystick input
    # Game should be over
    # TODO drain output and update score
    print(f"Part two score is {score}")

part2(ls)
