# Variables
CC = gcc
CFLAGS = -Wall -g -c
ODUMP = objdump
ODFLAGS = -d

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

# Objects list
objects = $(patsubst $(SRCDIR)/%.c,$(OBJDIR)/%.o,$(sources))

# Tests
tests := $(wildcard $(TSTDIR)/*.c)

# Tests list
tests_list := $(patsubst $(TSTDIR)/%.c,$(OBJDIR)/%.exe,$(tests))

# run rules.
run: $(TGTDIR)/$(target)
	cd $(TGTDIR) && ./$(target)

$(TGTDIR)/$(target) : $(objects)
	$(CC) -o $@ $^

# test rules.
test: $(tests_list)
	$<

$(OBJDIR)/%.exe: $(TSTDIR)/%.c $(objects) | $(objects)
	$(CC) -o $@ $< $(filter $(@D)/$(basename $(@F))/$(basename $(@F)).o,$(objects))

# shared build rules.
$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	@mkdir -p $(@D) $(DMPDIR)/$(@D)
	$(CC) $(CFLAGS) -o $@ $<
	$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@.list

folders:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(TGTDIR)
	@mkdir -p $(DMPDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR) $(DMPDIR)
