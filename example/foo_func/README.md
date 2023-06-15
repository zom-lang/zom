# Example: Calling Zom code from C

In `test.zom`, there is a function, that takes a int in parameters and return the sum of 123 and the args passed.
`main.c` call `foo(..)` (the Zom function) and print the result to stdout.

## Run the example
To run this example you need `make`, `gcc`, `rust` and `cargo`.

Just run `make` in this directory and you should see a similar output:
```bash
$ make
cargo run -r -q -- bobj test.zom -o output.o
Wrote result to "output.o"!
rm -f prog
gcc -g -c main.c
gcc -g -o prog main.o output.o
chmod +x prog
./prog
Example, call Zom from C!
foo = 444
```

At the last line we can see *(if you don't please open an issue on the GitHub repo)* `foo = 444`, that the correct answer 123 + 321 = 444.