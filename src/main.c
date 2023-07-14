#include <stdio.h>

#include "impl.h"
#include "turing.h"

int main(int argc, char *argv[]) {
	// fail if no word is given as argv[1]
	if (argc < 2) {
		printf("usage: %s word\n", argv[0]);
		return 1;
	}

	struct turingmachine *tm = create_turingmachine(accept1, delta1);
	process_word(tm, argv[1]);
	delete_turinmachine(tm);

	return 0;
}
