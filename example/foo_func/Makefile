CC      = gcc
CFLAGS  = -g
RM      = rm -f


default: all

all: buildZom clean prog chmod run

prog: main.c
	$(CC) $(CFLAGS) -c main.c
	$(CC) $(CFLAGS) -o prog main.o output.o

run: main.c
	./prog

chmod:
	chmod +x prog

buildZom:
	cargo run -r -q -- bobj test.zom -o output.o

clean:
	$(RM) prog