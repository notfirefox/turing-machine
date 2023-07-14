#pragma once

#include <stdbool.h>

// forward declaration
struct tape;

// create a new tape with a given blank symbol
struct tape *create_tape(const char);

// move to the left
void move_left(struct tape *);

// move to the right
void move_right(struct tape *);

// read the element you are currently pointing at
char read_element(const struct tape *);

// write to the element you are currently pointing at
void write_element(struct tape *, const char);

// print a tape
void print_tape(const struct tape *);

// load a word onto the tape
void prepare_tape(struct tape *, const char *const);

// delete an existing tape
void delete_tape(struct tape *);
