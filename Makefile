.POSIX:
.SUFFIXES:

CC		= cc
CFLAGS		= -W -O
BUILDDIR	= build
SRCDIR		= src

# List your source files and corresponding object files here
SOURCES  = impl.c main.c tape.c turing.c  # Add your source files here
OBJECTS  = $(BUILDDIR)/impl.o $(BUILDDIR)/main.o $(BUILDDIR)/tape.o $(BUILDDIR)/turing.o  # Corresponding .o files

# Default target
all: $(BUILDDIR) main

# Create the build directory if it doesn't exist
$(BUILDDIR):
	mkdir -p $(BUILDDIR)

# Main target
main: $(OBJECTS)
	@echo "Linking: $@ with objects: $(OBJECTS)"
	$(CC) -o $(BUILDDIR)/main $(OBJECTS)

# Object file targets
$(BUILDDIR)/impl.o: $(SRCDIR)/impl.c $(SRCDIR)/impl.h
	@echo "Compiling: $(SRCDIR)/impl.c to $@"
	$(CC) -c $(CFLAGS) $(SRCDIR)/impl.c -o $@

$(BUILDDIR)/main.o: $(SRCDIR)/main.c
	@echo "Compiling: $(SRCDIR)/main.c to $@"
	$(CC) -c $(CFLAGS) $(SRCDIR)/main.c -o $@

$(BUILDDIR)/tape.o: $(SRCDIR)/tape.c $(SRCDIR)/tape.h
	@echo "Compiling: $(SRCDIR)/tape.c to $@"
	$(CC) -c $(CFLAGS) $(SRCDIR)/tape.c -o $@

$(BUILDDIR)/turing.o: $(SRCDIR)/turing.c $(SRCDIR)/turing.h
	@echo "Compiling: $(SRCDIR)/turing.c to $@"
	$(CC) -c $(CFLAGS) $(SRCDIR)/turing.c -o $@

# Clean target
clean:
	rm -f $(BUILDDIR)/*
