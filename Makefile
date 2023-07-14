.POSIX:
.SUFFIXES:

CC		= cc
CFLAGS		= -W -O
BUILDDIR	= build
SRCDIR		= src
PREFIX		= /usr/local

# List of object files
OBJECTS  = $(BUILDDIR)/hash.o \
	   $(BUILDDIR)/impl.o \
	   $(BUILDDIR)/main.o \
	   $(BUILDDIR)/tape.o \
	   $(BUILDDIR)/turing.o

# Default target
all: $(BUILDDIR) $(BUILDDIR)/main

# Create the build directory if it doesn't exist
$(BUILDDIR):
	mkdir -p $(BUILDDIR)

# Main target
$(BUILDDIR)/main: $(OBJECTS)
	$(CC) -o $(BUILDDIR)/main $(OBJECTS)

# Object file targets
$(BUILDDIR)/hash.o: $(SRCDIR)/hash.c $(SRCDIR)/hash.h
	$(CC) -c $(CFLAGS) $(SRCDIR)/hash.c -o $@

$(BUILDDIR)/impl.o: $(SRCDIR)/impl.c $(SRCDIR)/impl.h
	$(CC) -c $(CFLAGS) $(SRCDIR)/impl.c -o $@

$(BUILDDIR)/main.o: $(SRCDIR)/main.c
	$(CC) -c $(CFLAGS) $(SRCDIR)/main.c -o $@

$(BUILDDIR)/tape.o: $(SRCDIR)/tape.c $(SRCDIR)/tape.h
	$(CC) -c $(CFLAGS) $(SRCDIR)/tape.c -o $@

$(BUILDDIR)/turing.o: $(SRCDIR)/turing.c $(SRCDIR)/turing.h
	$(CC) -c $(CFLAGS) $(SRCDIR)/turing.c -o $@

# Install target
install: $(BUILDDIR)/main
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	mkdir -p $(DESTDIR)$(PREFIX)/share/man/man1
	cp -f $(BUILDDIR)/main $(DESTDIR)$(PREFIX)/bin/tm
	gzip < tm.1 > $(DESTDIR)$(PREFIX)/share/man/man1/tm.1.gz

# Uninstall target
uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/tm
	rm -f $(DESTDIR)$(PREFIX)/share/man/man1/tm.1.gz

# Clean target
clean:
	rm -f $(BUILDDIR)/*
