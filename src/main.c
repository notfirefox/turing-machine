#include <stdint.h>
#include <stdio.h>

#include "impl.h"
#include "turing.h"

static const uint32_t FNV_PRIME = 0x01000193;
static const uint32_t FNV_OFFSET_BASIS = 0x811c9dc5;

// may be used in the future for a hash table, e.g. fnv1a32("q0,0")
static uint32_t fnv1a32(const char *const buffer) {
	uint32_t hash = FNV_OFFSET_BASIS;

	const char *ptr = buffer;
	while (*ptr) {
		hash = (hash ^ *ptr) * FNV_PRIME;
		ptr++;
	}
	return hash;
}

int main(void) {
	char word[256];
	printf("Enter word: ");
	scanf("%255s", word);

	struct turingmachine *tm = create_turingmachine(accept1, delta1);
	process_word(tm, word);
	delete_turinmachine(tm);

	return 0;
}
