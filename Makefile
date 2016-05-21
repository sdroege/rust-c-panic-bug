all: test

libtest.so: test.rs
	rustc -g --crate-type dylib -o $@ $<

test: test.c libtest.so
	gcc -g -Wall -o test test.c -L. -ltest

test2: test2.rs
	rustc -g -o $@ $<

run: test
	LD_LIBRARY_PATH=`pwd` ./test

run2: test2
	./test2

clean:
	rm -f libtest.so test test2
