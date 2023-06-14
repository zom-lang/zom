CC      = gcc
CFLAGS  = -g
RM      = rm -f


default: all

all: buildMona clean prog chmod run

prog: main.c
	$(CC) $(CFLAGS) -c main.c
	$(CC) $(CFLAGS) -o prog main.o output.o

run: main.c
	./prog

chmod:
	chmod +x prog

buildMona:
	cargo run -r -q -- bobj test.mn -o output.o

clean:
	$(RM) prog