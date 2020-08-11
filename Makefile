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

# Sources list
sources := $(wildcard $(SRCDIR)/*.c $(SRCDIR)/*/*.c)

# Objects list
objects = $(patsubst $(SRCDIR)/%.c,$(OBJDIR)/%.o,$(sources))

# Tests list
tests := $(wildcard $(TSTDIR)/*.c)
# todo:: try building tests list like objects, then use that as the prereq for test.

# main rule.
run: $(TGTDIR)/$(target)
	cd $(TGTDIR) && ./$(target)

# test rule.
test: $(TSTDIR)/%.exe
	$<
# shared rules.
$(TGTDIR)/$(target) : $(objects)
	$(CC) -o $@ $^

$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	@mkdir -p $(@D) $(DMPDIR)/$(@D)
	$(CC) $(CFLAGS) -o $@ $<
	$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@.list

# tests build.
$(TSTDIR)/%.exe : $(TSTDIR)/%.c | $(OBJDIR)/%.o
	$(CC) $(CFLAGS) -o $@ $(objects)

folders:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(TGTDIR)
	@mkdir -p $(DMPDIR)/$(OBJDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR) $(DMPDIR)
