CC      = gcc
CFLAGS  = -g
RM      = rm -f


default: all

all: prog chmod run

prog: main.c
	$(CC) $(CFLAGS) -c main.c
	$(CC) $(CFLAGS) -o prog main.o output.o # Where output.o is the Mona obj

run: main.c
	echo 'Execution of the program'
	./prog

chmod:
	chmod +x prog

clean:
	$(RM) prog