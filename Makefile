.PHONY: all clean test

all: clean test

clean:
	rm lua/harbinger.so

test:
	./test.sh
