class Imac:
    STOP = 99

    # operations
    def add(self, str_op):
        if self.i+5 > len(self.pg):
            raise ValueError(f"OOB after add instruction at {self.i}")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in add at {self.i}")
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in add at {self.i}")
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        elif str_op[1] == "1":
            idx2 = self.pg[self.i+2]
        elif str_op[1] == "2":
            if self.relative_base + self.pg[self.i+2] < 0:
                raise ValueError(f"Rel idx negative in add at {self.i}")
            idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
        else:
            raise ValueError(f"Unknown mode in add at {self.i}")
        if str_op[0] == "0":
            self.pg[self.pg[self.i+3]] = idx1 + idx2
        elif str_op[0] == "1":
            raise ValueError("Can't handle a immediate mode add result")
        elif str_op[0] == "2":
            if self.relative_base + self.pg[self.i+3] < 0:
                raise ValueError(f"Rel idx negative in add at {self.i}")
            self.pg[self.relative_base + self.pg[self.i+3]] = idx1 + idx2
        else:
            raise ValueError(f"Unknown mode in add at {self.i}")
        self.i += 4

    def mul(self, str_op):
        if self.i+5 > len(self.pg):
            raise ValueError(f"OOB after mul instruction at {self.i}")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in mul at {self.i}")
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in mul at {self.i}")
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        elif str_op[1] == "1":
            idx2 = self.pg[self.i+2]
        elif str_op[1] == "2":
            if self.relative_base + self.pg[self.i+2] < 0:
                raise ValueError(f"Rel idx negative in mul at {self.i}")
            idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
        else:
            raise ValueError(f"Unknown mode in mul at {self.i}")
        if str_op[0] == "0":
            self.pg[self.pg[self.i+3]] = idx1 * idx2
        elif str_op[0] == "1":
            raise ValueError("Can't handle a immediate mode mul result")
        elif str_op[0] == "2":
            if self.relative_base + self.pg[self.i+3] < 0:
                raise ValueError(f"Rel idx negative in mul at {self.i}")
            self.pg[self.relative_base + self.pg[self.i+3]] = idx1 * idx2
        else:
            raise ValueError(f"Unknown mode in mul at {self.i}")
        self.i += 4

    def iin(self, str_op):
        if self.i + 3 > len(self.pg):
            raise ValueError(f"OOB after iin instruction at {self.i}")
        if str_op[2] == "0":
            idxr = self.pg[self.i+1]
        elif str_op[2] == "1":
            raise ValueError("Can't handle a immediate mode iin result")
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in iin at {self.i}")
            idxr = self.relative_base + self.pg[self.i+1]
        else:
            raise ValueError(f"Unknown mode in iin at {self.i}")
        if len(self.inputs) > 0:
            i_in = int(self.inputs.pop(0))
            self.pg[idxr] = i_in
            self.i += 2
        else:
            self.running = 2

    def iout(self, str_op):
        if self.i+3 > len(self.pg):
            raise ValueError(f"OOB after iout instruction at {self.i}")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in iout at {self.i}")
        self.outputs.append(f"{idx1}")
        self.i += 2

    def jit(self, str_op): # jump-if-true
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in jit at {self.i}")
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in for p1 jit at {self.i}")
        if idx1 != 0:
            if str_op[1] == "0":
                idx2 = self.pg[self.pg[self.i+2]]
            elif str_op[1] == "1":
                idx2 = self.pg[self.i+2]
            elif str_op[1] == "2":
                idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
            else:
                raise ValueError(f"Unknown mode for p2 in jit at {self.i}")
            self.i = idx2
        else:
            self.i += 3

    def jif(self, str_op): # jump-if-false
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in jif at {self.i}")
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in for p1 jif at {self.i}")
        if idx1 == 0:
            if str_op[1] == "0":
                idx2 = self.pg[self.pg[self.i+2]]
            elif str_op[1] == "1":
                idx2 = self.pg[self.i+2]
            elif str_op[1] == "2":
                idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
            else:
                raise ValueError(f"Unknown mode for p2 in jif at {self.i}")
            self.i = idx2
        else:
            self.i += 3

    def lt(self, str_op): # less than
        if self.i+5 > len(self.pg):
            raise ValueError(f"Will go OOB after {self.i}, lt op")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in for p1 lt at {self.i}")
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        elif str_op[1] == "1":
            idx2 = self.pg[self.i+2]
        elif str_op[1] == "2":
            idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
        else:
            raise ValueError(f"Unknown mode in for p2 lt at {self.i}")
        if str_op[0] == "0":
            idxr = self.pg[self.i+3]
        elif str_op[0] == "1":
            raise ValueError(f"No immediate mode for writes, lt op")
        elif str_op[0] == "2":
            idxr = self.relative_base + self.pg[self.i+3]
        else:
            raise ValueError(f"Unknown mode in for p3 lt at {self.i}")
        if idx1 < idx2:
            self.pg[idxr] = 1
        else:
            self.pg[idxr] = 0
        self.i += 4

    def eq(self, str_op): # equals
        if self.i+5 > len(self.pg):
            raise ValueError(f"Will go OOB after {self.i}, eq op")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in for p1 eq at {self.i}")
        if str_op[1] == "0":
            idx2 = self.pg[self.pg[self.i+2]]
        elif str_op[1] == "1":
            idx2 = self.pg[self.i+2]
        elif str_op[1] == "2":
            idx2 = self.pg[self.relative_base + self.pg[self.i+2]]
        else:
            raise ValueError(f"Unknown mode in for p2 eq at {self.i}")
        if str_op[0] == "0":
            idxr = self.pg[self.i+3]
        elif str_op[0] == "1":
            raise ValueError(f"No immediate mode for writes, eq op")
        elif str_op[0] == "2":
            idxr = self.relative_base + self.pg[self.i+3]
        else:
            raise ValueError(f"Unknown mode in for p3 eq at {self.i}")
        if idx1 == idx2:
            self.pg[idxr] = 1
        else:
            self.pg[idxr] = 0
        self.i += 4

    def rbo(self, str_op): # relative base offset 
        if self.i+3 > len(self.pg):
            raise ValueError(f"Will go OOB after {self.i}, lt op")
        if str_op[2] == "0":
            idx1 = self.pg[self.pg[self.i+1]]
        elif str_op[2] == "1":
            idx1 = self.pg[self.i+1]
        elif str_op[2] == "2":
            if self.relative_base + self.pg[self.i+1] < 0:
                raise ValueError(f"Rel idx negative in mul at {self.i}")
            idx1 = self.pg[self.relative_base + self.pg[self.i+1]]
        else:
            raise ValueError(f"Unknown mode in rbo op at {self.i}")
        self.relative_base = self.relative_base + idx1
        self.i += 2

    def __init__(self, pg, i, inputs):
        self.pg = pg.copy()
        # add working memory to the end of the program with 0s
        #  guessing 1M is enough
        self.pg += [0] * 1000000
        self.i = i
        self.relative_base = 0
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
        8 : eq,
        9 : rbo
    }
    
    def run(self, dbg_prints = False):
        self.running = 1
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
                Imac.OP_CODES[op](self, str_op)
                self.num_ops += 1

        return(self.running)