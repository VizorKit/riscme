# Variables
CC = gcc
CFLAGS = -Wall -g -c

# Directories
OBJDIR := obj
TGTDIR := target
SRCDIR := src
DMPDIR := dump

# Sources list
SRCS := main.c kernel.c
VPATH := src:../headers

objects = $(notdir $(patsubst %.c,%.o,$(addprefix $(SRCDIR)/,$(SRCS))))
prefix_objs = $(addprefix $(OBJDIR)/, $(objects))
target = main

$(target) : $(objects)
	$(CC) -o $(TGTDIR)/$@ $(prefix_objs)

%.o: %.c | folders
	$(CC) $(CFLAGS) -o $(OBJDIR)/$@ $<

# $(target): $(objects)
# 	$(CC) -o $(TGTDIR)/$(target) $(OBJDIR)/$(objects)

# $(objects): $(sources) | folders
# 	$(CC) $(CFLAGS) $< -o $(OBJDIR)/$@

folders:
	mkdir -p $(OBJDIR)
	mkdir -p $(TGTDIR)
	mkdir -p $(DMPDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR) $(DMPDIR)
