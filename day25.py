from imac import Imac

filename = "input25.txt"
do_not_take = ["escape pod", "molten lava"]

with open(filename) as f:
    ls = f.read().strip().split(',')

p = [int(x) for x in ls]
vm = Imac(p, 0, [])
vm.run()
done = False

while not done:
    room_out = vm.get_output()
    room_text = []
    for c in room_out:
        room_text.append(chr(int(c)))
    room_text = ''.join(room_text)
    print(room_text)
    if "Command?" not in room_text:
        done = True

    # get keyboard input
    kbd_in = input("=> ")
    if kbd_in.startswith('w'):
        vm_in = "north"
    elif kbd_in.startswith('d'):
        vm_in = "east"
    elif kbd_in.startswith('s'):
        vm_in = "south"
    elif kbd_in.startswith('a'):
        vm_in = "west"
    elif kbd_in.startswith('pu'):
        parts = kbd_in.split()
        vm_in = f"take {' '.join(parts[1:])}"
    elif kbd_in.startswith('pd'):
        parts = kbd_in.split()
        vm_in = f"drop {' '.join(parts[1:])}"
    elif kbd_in.startswith('i'):
        vm_in = "inv"
    else:
        print(f"{kbd_in} not a valid command")
    
    # text to ints
    cur_command = []
    for c in vm_in:
        cur_command.append(ord(c))
    cur_command.append(10)
    vm.add_input(cur_command)
    vm.run()

print("Hope we got the password")
