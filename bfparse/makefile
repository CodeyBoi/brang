OBJS=main.o chunk.o darray.o scanner.o debug.o constval.o vm.o parser.o bf.o
CC=gcc
CFLAGS=-g
OUT=app

$(OUT): $(OBJS)
	$(CC) -o $(OUT) $(OBJS)
 
clean:
	rm -f $(OBJS)
	rm -f $(OUT)
