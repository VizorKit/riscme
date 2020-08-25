# Variables
CC = gcc
CFLAGS = -Wall -O3 -std=c99 -c
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
	$(CC) $^ -o $@

# test rules.
test: $(tests_list) | $(objects)
	$<
	@echo $< "success"

$(OBJDIR)/%.exe: $(TSTDIR)/%.c $(objects) | $(objects)
	@echo $(filter $(@D)/$(basename $(@F))/$(basename $(@F)).o,$(objects))
	$(CC) -o $@ $< $(filter $(@D)/$(basename $(@F))/$(basename $(@F)).o,$(objects))

# shared build rules.
$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	@mkdir -p $(@D)
	@$(CC) $(CFLAGS) -o $@ $<

folders:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(TGTDIR)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(TGTDIR)
