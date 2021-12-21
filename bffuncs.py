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


    def movptr(self, adr):
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
        self.movptr(adr)
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


    def cpy(self, src, dest):
        """
        Copies the value at `src` into `dest`.
        """
        if src == dest:
            raise ValueError("can't copy a register to itself")
        temp = self.calloc(1)
        self.setval(dest, 0)
        # Sets `dest` and `temp` to `src` and `src` to zero
        self.movptr(src)
        print('[-', end='')
        self.movptr(dest)
        print('+', end='')
        self.movptr(temp)
        print('+', end='')
        self.movptr(src)
        print(']')

        # Sets `src` to `temp` and `temp` to zero
        self.movptr(temp)
        print('[-', end='')
        self.movptr(src)
        print('+', end='')
        self.movptr(temp)
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
        self.movptr(self.env[varname])
        print('.', end='')


    def add(self, lhs, rhs, out):
        """
        Adds the value at `adr1` to `adr2` and writes it to `out`.
        """
        if lhs is str:
            lhs = self.env[lhs]
        if rhs is str: 
            rhs = self.env[lhs]
        left = self.malloc(1)
        right = self.malloc(1)
        self.cpy(lhs, left)
        self.cpy(rhs, right)
        self.cpy(left, out)
        self.movptr(right)
        print('[-', end='')
        self.movptr(out)
        print('+', end='')
        self.movptr(right)
        print(']')
        self.dealloc(left)
        self.dealloc(right)
