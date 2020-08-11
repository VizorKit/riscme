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

# Sources list
SRCS := main.c kernel.c
# Sources location
VPATH := src:../headers

# Objects list
objects = $(notdir $(patsubst %.c,%.o,$(addprefix $(SRCDIR)/,$(SRCS))))
# Objects list with prefix
prefix_objs = $(addprefix $(OBJDIR)/, $(objects))

# Target name
target = main

# main rule.
run: $(TGTDIR)/$(target)
	cd $(TGTDIR) && ./$(target)

# test rule.

# shared rules.
$(TGTDIR)/$(target) : $(prefix_objs)
	$(CC) -o $@ $(prefix_objs)

$(OBJDIR)/%.o: %.c | folders
	$(CC) $(CFLAGS) -o $@ $<
	$(ODUMP) $(ODFLAGS) $@ > $(DMPDIR)/$@

# test runs.

folders:
	mkdir -p $(OBJDIR)
	mkdir -p $(TGTDIR)
	mkdir -p $(DMPDIR)/$(OBJDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR) $(DMPDIR)
