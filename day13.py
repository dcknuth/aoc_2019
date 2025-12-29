from imac import Imac
from copy import deepcopy
from time import sleep

filename = "input13.txt"
with open(filename) as f:
    ls = f.read().strip()

def print_screen(screen):
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

def part1(ls):
    plist = [int(x) for x in ls.split(',')]
    cur_mac = Imac(plist, 0, [])
    end_state = cur_mac.run()
    block_tile_count = 0
    for i, cur_block in enumerate(cur_mac.outputs):
        if (i+1)%3 == 0 and cur_block == '2':
            block_tile_count += 1
    print(f"Part one block count is: {block_tile_count}")

#part1(ls)

def find_ball(outputs):
    for i in range(0, len(outputs), 3):
        x = int(outputs[i])
        y = int(outputs[i+1])
        tile_id = outputs[i+2]
        if tile_id == '4':
            return(x, y)

def process_output(outputs, screen, score, ball, ball_dir,
                       paddle, walls, old_ball):
    for i in range(0, len(outputs), 3):
        x = int(outputs[i])
        y = int(outputs[i+1])
        tile_id = outputs[i+2]
        if x < 0:
            score[0] = tile_id
        else:
            screen[(x, y)] = tile_id
            if tile_id == '4':
                old_ball[0] = ball[0]
                old_ball[1] = ball[1]
                ball[0] = x
                ball[1] = y
                if old_ball[0] > -1:
                    ball_dir[0] = ball[0] - old_ball[0]
                    ball_dir[1] = ball[1] - old_ball[1]
            elif tile_id == '3':
                paddle[0] = x
                paddle[1] = y
    walls[0] = min([k[0] for k in screen.keys()])
    walls[1] = max([k[0] for k in screen.keys()])

def move_js(paddle, future_ball):
    if future_ball[0] > paddle[0]:
        return(1)
    if future_ball[0] < paddle[0]:
        return(-1)
    return(0)

def run_sub_game(mac, screen, paddle):
    sub_mac = deepcopy(mac)
    sub_state = 0
    sub_screen = screen.copy()
    score = [0]
    ball = [-1, -1]
    ball_dir = [0, 0]
    sub_paddle = paddle.copy()
    walls = [-1, -1] # left wall x, right wall x
    old_ball = [-1, -1]
    while sub_state != -1:
        # feed in a 0 js move
        sub_mac.add_input([0])
        sub_state = sub_mac.run()
        outputs = sub_mac.get_output()
        process_output(outputs, sub_screen, score, ball, ball_dir,
                       paddle, walls, old_ball)
        if ball[1] == paddle[1] - 1:
            return(ball)
    # if we get here, game is won, return ball x and paddle y -1
    return([ball[0], paddle[1]-1])


def part2(ls):
    plist = [int(x) for x in ls.split(',')]
    mac = Imac(plist, 0, [])
    mac.pg[0] = 2 # set to play for free
    mac_state = 0 # waiting to run
    screen = dict()
    score = [0]
    ball = [-1, -1]
    ball_dir = [0, 0]
    paddle = [-1, -1]
    walls = [-1, -1] # left wall x, right wall x
    old_ball = [-1, -1]
    have_future = False
    while mac_state != -1:
        mac_state = mac.run()
        # should have a screen of output and be waiting for input
        outputs = mac.get_output()
        process_output(outputs, screen, score, ball, ball_dir,
                       paddle, walls, old_ball)
        print_screen(screen)
        print(score)
        sleep(0.02)
        # see if we need a future ball
        if old_ball[1] == paddle[1] - 1 and ball_dir[1] < 0:
            have_future = False
        if not have_future:
            # run until the ball gets to the line above the paddle line
            future_ball = run_sub_game(mac, screen, paddle)
            have_future = True
        js_input = move_js(paddle, future_ball)
        mac.add_input([js_input])
    # Game should be over
    process_output(outputs, screen, score, ball, ball_dir,
                       paddle, walls, old_ball)
    print(f"Part two score is {score}")

part2(ls)
