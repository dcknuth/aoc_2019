class Imac:
    STOP = 99

    # operations
    def add(self, str_op, inputs, outputs):
        if self.i + 5 > len(self.pg):
            print(f"Will go OOB after add instruction at {self.i}")
            exit(0)
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        else:
            idx2 = self.pg[self.i+2]
        if str_op[0] == "0":
            self.pg[self.pg[self.i+3]] = idx1 + idx2
        else:
            raise ValueError("Can't handle a immediate mode add result")
        self.i += 4

    def mul(self, str_op, inputs, outputs):
        if self.i+5 > len(self.pg):
            print(f"Will go OOB after mul instruction at {self.i}")
            exit(0)
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        else:
            idx2 = self.pg[self.i+2]
        if str_op[0] == "0":
            self.pg[self.pg[self.i+3]] = idx1 * idx2
        else:
            raise ValueError("Can't handle a immediate mode mul result")
        self.i += 4

    def iin(self, str_op, inputs, outputs):
        if self.i + 3 > len(self.pg):
            print(f"Will go OOB after iin instruction at {self.i}")
            exit(0)
        if str_op[2] == "0":
            idxr = self.pg[self.i+1]
        else:
            raise ValueError("Can't handle a immediate mode iin result")
        if len(self.inputs) > 0:
            i_in = int(self.inputs.pop(0))
            self.pg[idxr] = i_in
            self.i += 2
        else:
            self.running = 2

    def iout(self, str_op, inputs, outputs):
        if self.i+3 > len(self.pg):
            print(f"Will go OOB after iout instruction at {self.i}")
            exit(0)
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        #print(f"{idx1}")
        outputs.append(f"{idx1}")
        self.i += 2

    def jit(self, str_op, inputs, outputs): # jump-if-true
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if idx1 != 0:
            if str_op[1] == "0":
                idx2 = self.pg[self.pg[self.i+2]]
            else:
                idx2 = self.pg[self.i+2]
            self.i = idx2
        else:
            self.i += 3

    def jif(self, str_op, inputs, outputs): # jump-if-false
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if idx1 == 0:
            if str_op[1] == "0":
                idx2 = self.pg[self.pg[self.i+2]]
            else:
                idx2 = self.pg[self.i+2]
            self.i = idx2
        else:
            self.i += 3

    def lt(self, str_op, inputs, outputs): # less than
        if self.i+5 > len(self.pg):
            raise ValueError(f"Will go OOB after {self.i}, lt op")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        else:
            idx2 = self.pg[self.i+2]
        if str_op[0] == "0":
            idxr = self.pg[self.i+3]
        else:
            raise ValueError(f"No immediate mode for writes, lt op")
        if idx1 < idx2:
            self.pg[idxr] = 1
        else:
            self.pg[idxr] = 0
        self.i += 4

    def eq(self, str_op, inputs, outputs): # equals
        if self.i+5 > len(self.pg):
            raise ValueError(f"Will go OOB after {self.i}, lt op")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        else:
            idx1 = self.pg[self.i+1]
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        else:
            idx2 = self.pg[self.i+2]
        if str_op[0] == "0":
            idxr = self.pg[self.i+3]
        else:
            raise ValueError(f"No immediate mode for writes, eq op")
        if idx1 == idx2:
            self.pg[idxr] = 1
        else:
            self.pg[idxr] = 0
        self.i += 4

    def __init__(self, pg, i, inputs):
        self.pg = pg.copy()
        self.i = i
        self.inputs = inputs.copy()
        self.outputs = []
        # 0=not running 1=running 2=waiting input -1=exited
        self.running = 0
        self.num_ops = 0

    def add_input(self, inputs):
        self.inputs.extend(inputs)
    
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
    
    def run(self, inputs = None, outputs = None, dbg_prints = False):
        self.running = 1
        if inputs != None and len(inputs) > 0:
            self.inputs.extend(inputs)
            inputs = []
        while self.running == 1:
            if dbg_prints:
                print(f"Running {self.num_ops=} Current index: {self.i}",
                    f"Val at cur index: {self.pg[self.i]}")
            # get expanded string op
            str_op = f"{self.pg[self.i]:05}"
            op = int(str_op[-2:])
            if op == Imac.STOP:
                self.running = -1
            elif op not in Imac.OP_CODES:
                print(f"Error: unknown opcode: {self.pg[self.i]}/{op}",
                    f"at index {self.i}")
                self.running = -1
            else:
                Imac.OP_CODES[op](self, str_op, inputs, outputs)
                self.num_ops += 1

        return(self.running)