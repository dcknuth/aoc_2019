# operations
def add(pg, index, str_op):
    i = index[0]
    if i+5 > len(pg):
        print(f"Will go OOB after add instruction at {i}")
        exit(0)
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if str_op[1] == "0":
        idx2 = pg[pg[i+2]]
    else:
        idx2 = pg[i+2]
    if str_op[0] == "0":
        pg[pg[i+3]] = idx1 + idx2
    else:
        raise ValueError("Can't handle a immediate mode add result")
    index[0] = i + 4

def mul(pg, index, str_op):
    i = index[0]
    if i+5 > len(pg):
        print(f"Will go OOB after mul instruction at {i}")
        exit(0)
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if str_op[1] == "0":
        idx2 = pg[pg[i+2]]
    else:
        idx2 = pg[i+2]
    if str_op[0] == "0":
        pg[pg[i+3]] = idx1 * idx2
    else:
        raise ValueError("Can't handle a immediate mode mul result")
    index[0] = i + 4

def iin(pg, index, str_op):
    i = index[0]
    if i+3 > len(pg):
        print(f"Will go OOB after iin instruction at {i}")
        exit(0)
    if str_op[2] == "0":
        idxr = pg[i+1]
    else:
        raise ValueError("Can't handle a immediate mode iin result")
    i_in = int(input("Input an integer: "))
    pg[idxr] = i_in
    index[0] = i + 2

def iout(pg, index, str_op):
    i = index[0]
    if i+3 > len(pg):
        print(f"Will go OOB after iout instruction at {i}")
        exit(0)
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    print(f"{idx1}")
    index[0] = i + 2

def jit(pg, index, str_op): # jump-if-true
    i = index[0]
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if idx1 != 0:
        if str_op[1] == "0":
            idx2 = pg[pg[i+2]]
        else:
            idx2 = pg[i+2]
        index[0] = idx2
    else:
        index[0] = i + 3

def jif(pg, index, str_op): # jump-if-false
    i = index[0]
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if idx1 == 0:
        if str_op[1] == "0":
            idx2 = pg[pg[i+2]]
        else:
            idx2 = pg[i+2]
        index[0] = idx2
    else:
        index[0] = i + 3

def lt(pg, index, str_op): # less than
    i = index[0]
    if i+5 > len(pg):
        raise ValueError(f"Will go OOB after {i}, lt op")
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if str_op[1] == "0":
        idx2 = pg[pg[i+2]]
    else:
        idx2 = pg[i+2]
    if str_op[0] == "0":
        idxr = pg[i+3]
    else:
        raise ValueError(f"No immediate mode for writes, lt op")
    if idx1 < idx2:
        pg[idxr] = 1
    else:
        pg[idxr] = 0
    index[0] = i + 4

def eq(pg, index, str_op): # equals
    i = index[0]
    if i+5 > len(pg):
        raise ValueError(f"Will go OOB after {i}, lt op")
    if str_op[2] == "0":
        idx1 = pg[pg[i+1]]
    else:
        idx1 = pg[i+1]
    if str_op[1] == "0":
        idx2 = pg[pg[i+2]]
    else:
        idx2 = pg[i+2]
    if str_op[0] == "0":
        idxr = pg[i+3]
    else:
        raise ValueError(f"No immediate mode for writes, eq op")
    if idx1 == idx2:
        pg[idxr] = 1
    else:
        pg[idxr] = 0
    index[0] = i + 4

OP_CODES = {
    1 : add,
    2 : mul,
    3 : iin,
    4 : iout,
    5 : jit,
    6 : jif,
    7 : lt,
    8 : eq
}

STOP = 99

def imac(pg, i, dbg_prints = False):
    running = True
    exn = 0 # number of instructions that have run
    index = [i,] # cur index that can be modified in functions 
    while running:
        if dbg_prints:
            print(f"Running {exn=} Current index: {index[0]}",
                  f"Val at cur index: {pg[index[0]]}")
        # get expanded string op
        str_op = f"{pg[index[0]]:05}"
        op = int(str_op[-2:])
        if op == STOP:
            running = False
        elif op not in OP_CODES:
            print(f"Error: unknown opcode: {pg[index[0]]}/{op}",
                  f"at index {index[0]}")
            running = False
        else:
            OP_CODES[op](pg, index, str_op)
            exn += 1
