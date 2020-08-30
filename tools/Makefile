# Variables
CC = gcc
CFLAGS = -Wall -O0 -std=c11 -DDEBUG -g
ODUMP = objdump
ODFLAGS = -d
OBJCOPY = objcopy
OBJFLAGS = -O binary

# Directories
OBJDIR := obj
TGTDIR := target
SRCDIR := src
TSTDIR := tests

# Target name
target = vk

# Sources
sources := $(wildcard $(SRCDIR)/*.c $(SRCDIR)/*/*.c)

# Objects list
objects = $(patsubst $(SRCDIR)/%.c,$(OBJDIR)/%.o,$(sources))

# Tests
tests := $(wildcard $(TSTDIR)/*.c)

# Tests list
tests_list := $(patsubst $(TSTDIR)/%.c,$(OBJDIR)/%.exe,$(tests))

#Includes
includes := $(wildcard $(SRCDIR)/*.h $(SRCDIR)/*/*.h)

# run rules.
run: $(TGTDIR)/$(target)
	@echo "success"

$(TGTDIR)/$(target) : $(objects)
	$(CC) $(CFLAGS) $^ -o $@

# test rules.
test: $(tests_list)
	@$(foreach f,$^,./$(f) && echo $(f) "success";)

$(OBJDIR)/%.exe: $(TSTDIR)/%.c $(objects)
	$(CC) $(CFLAGS) -o $@ $< $(filter $(@D)/$(basename $(@F))/%.o,$(objects))

# shared build rules.
$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	@mkdir -p $(@D)
	@$(CC) $(CFLAGS) -c -o $@ $<

folders:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(TGTDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR)
