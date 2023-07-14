#pragma once

#include <stdbool.h>

// blank symbol the turing machine
static const char BLANK = '*';

// forward declaration
struct turingmachine;

// function pointer for accept function
typedef bool (*accept_fn)(const int);

// parameter type for delta function
struct delta_param {
	const int state;
	const char input;
};

// move operation for delta result
enum move { right, left, none };

// return type for delta function
struct delta_result {
	int state;
	char output;
	enum move move;
};

// function pointer for delta function
typedef struct delta_result (*delta_fn)(struct delta_param);

// create a new turing machine
struct turingmachine *create_turingmachine(const accept_fn, const delta_fn);

// process a given word and return whether the turing machine accepts it
bool process_word(struct turingmachine *, const char *const);

// delete an existing turing machine
void delete_turinmachine(struct turingmachine *);
