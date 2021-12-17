from collections import namedtuple

class BrainFuck:

    Var = namedtuple('Var', ['type', 'adr'])
    LIMIT = 30000


    def __init__(self):
        self.ptr = 0
        self.env = {}
        self.allocd = {}


    def malloc(self, size):
        """
        Allocates the first found contigous sequence of length `size` with 
        free cells on the heap and returns the address to the first cell.
        """
        if size < 1:
            raise ValueError(f"can't allocate memory of size '{size}'")
        
        adr = 0
        memlen = 0
        while adr < self.LIMIT:
            if adr in self.allocd:
                adr += self.allocd[adr]
                memlen = 0
            else:
                adr += 1
                memlen += 1
            if memlen == size:
                adr = adr - memlen
                self.allocd[adr] = memlen
                return adr
        raise MemoryError('out of free cells')


    def calloc(self, size):
        """
        As `malloc`, but sets all cells to zero.
        """
        adr = self.malloc(size)
        for i in range(size):
            self.setval(adr + i, 0)
        return adr


    def dealloc(self, adr):
        if adr not in self.allocd:
            raise MemoryError('tried to deallocate nonallocated memory')
        self.allocd.pop(adr)


    def mov(self, adr):
        """
        Moves pointer to the address `adr`.
        """
        if not 0 <= adr < self.LIMIT:
            raise IndexError('pointer is out of bounds.')
        c = '>' if self.ptr < adr else '<'
        print(c * abs(self.ptr - adr), end='')
        self.ptr = adr
    

    def setval(self, adr, val):
        """
        Sets cell at `adr` to the value `val`.
        """
        self.mov(adr)
        print('[-]', end='')        # Sets cell to zero
        print('+' * val)            # Sets cell to `val`


    def assign(self, varname, val):
        """
        Assigns the value `val` to `var`.
        """
        # Finds the address of `varname` if it exists, or finds the first free cell
        if varname in self.env:
            adr = self.env[varname]
        else:
            adr = self.malloc(1)
            self.env[varname] = adr
        self.setval(adr, val)

    
    def free(self, varname):
        if varname not in self.env:
            raise KeyError('tried to free nonallocated variable')
        self.dealloc(self.env[varname])
        self.env.pop(varname)


    # a1, a2
    # mov(a1)
    # print('[-')
    # mov(a2)
    # print('-[')
    # mov(a1)


    def memcpy(self, src, dest):
        """
        Copies the value at `src` into `dest`.
        """
        if src == dest:
            raise ValueError("can't copy a register to itself")
        temp = self.calloc(1)
        self.setval(dest, 0)
        # Sets `dest` and `temp` to `src` and `src` to zero
        self.mov(src)
        print('[-', end='')
        self.mov(dest)
        print('+', end='')
        self.mov(temp)
        print('+', end='')
        self.mov(src)
        print(']')

        # Sets `src` to `temp` and `temp` to zero
        self.mov(temp)
        print('[-', end='')
        self.mov(src)
        print('+', end='')
        self.mov(temp)
        print(']')
        # Deallocates `temp`
        self.dealloc(temp)

    
    def puts(self, s):
        """
        Outputs a string literal `s` to stdout.
        """
        adr = self.malloc(1)
        for c in s:
            self.setval(adr, ord(c))
            print('.', end='')
        self.dealloc(adr)


    def putv(self, varname):
        if varname not in self.env:
            raise KeyError(f'variable {varname} was not defined')
        self.mov(self.env[varname])
        print('.', end='')


    def add(self, adr1, adr2, out):
        """
        Adds the value at `adr1` to `adr2` and writes it to `out`.
        """
        left = self.malloc(1)
        right = self.malloc(1)
        self.memcpy(adr1, left)
        self.memcpy(adr2, right)
        self.memcpy(left, out)
        self.mov(right)
        print('[-', end='')
        self.mov(out)
        print('+', end='')
        self.mov(right)
        print(']')
        self.dealloc(left)
        self.dealloc(right)


    def addvars(self, var1, var2, outvar):
        if var1 not in self.env:
            raise KeyError(f'variable {var1} was not defined')
        if var2 not in self.env:
            raise KeyError(f'variable {var2} was not defined')
        if outvar not in self.env:
            raise KeyError(f'variable {outvar} was not defined')
        adr1, adr2, adr3 = self.env[var1], self.env[var2], self.env[outvar]
        self.add(adr1, adr2, adr3)
        