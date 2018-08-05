NASM := nasm
NASM_FLAG := -f elf32

SRCS := $(wildcard *.S)
OBJS := $(patsubst %.S, %.o, $(SRCS))

.PHONY: build clean

build: $(OBJS)

%.o: %.S
	$(NASM) $(NASM_FLAG) -o $@ $<

clean:
	rm -rf $(OBJS)
