# Variables
CC = riscv64-unknown-elf-gcc
CFLAGS = -march=rv32imac -mabi=ilp32 -Wall -O2 -nostdlib -nostartfiles -ffreestanding  -c
LDFLAGS = -march=rv32imac -mabi=ilp32 -melf32lriscv -T
LD = riscv64-unknown-elf-ld
AS = riscv64-unknown-elf-as
ODUMP = riscv64-unknown-elf-objdump
ODFLAGS = -d
OBJCOPY = riscv64-unknown-elf-objcopy
OBJFLAGS = -O binary

# Directories
OBJDIR := obj
TGTDIR := target
SRCDIR := src
DMPDIR := dump
TSTDIR := tests

# Target name
target = main

# Sources
sources := $(wildcard $(SRCDIR)/*.c $(SRCDIR)/*/*.c)
asm_sources := $(wildcard $(SRCDIR)/*/*.s)

# Objects list
objects = $(patsubst $(SRCDIR)/%.c,$(OBJDIR)/%.o,$(sources))
objects += $(patsubst $(SRCDIR)/%.s,$(OBJDIR)/%.o,$(asm_sources))

# Tests
tests := $(wildcard $(TSTDIR)/*.c)

# Tests list
tests_list := $(patsubst $(TSTDIR)/%.c,$(OBJDIR)/%.exe,$(tests))

# run rules.
# todo:: setup qemu
run: $(TGTDIR)/$(target)
	@echo "success"

$(TGTDIR)/$(target) : $(objects)
	$(LD) $(LDFLAGS) memmap.ld $^ -o $@
	$(OBJCOPY) $@ $(OBJFLAGS) $@.bin 
	@mkdir -p $(@D) $(DMPDIR)/$(@D)
	@$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@.list

# test rules.
test: $(tests_list)
	$<

$(OBJDIR)/%.exe: $(TSTDIR)/%.c $(objects) | $(objects)
	$(CC) -o $@ $< $(filter $(@D)/$(basename $(@F))/$(basename $(@F)).o,$(objects))

# shared build rules.
$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	@mkdir -p $(@D) $(DMPDIR)/$(@D)
	@$(CC) $(CFLAGS) -o $@ $<
	@$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@.list

$(OBJDIR)/%.o: $(SRCDIR)/%.s | folders
	@mkdir -p $(@D) $(DMPDIR)/$(@D)
	@$(CC) $(CFLAGS) -o $@ $<
	@$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@.list


folders:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(TGTDIR)
	@mkdir -p $(DMPDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR) $(DMPDIR)
