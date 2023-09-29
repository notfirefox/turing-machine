#include "tape.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct tape {
    int index;
    char *ptr;
    int size;
    const char blank;
};

const static double OFFSET_FACTOR = 0.2;

struct tape *create_tape(const char blank) {
    struct tape *tape = malloc(sizeof(struct tape));
    if (tape == NULL) {
        printf("Memory allocation failed.\n");
        exit(EXIT_FAILURE);
        return NULL;
    }
    const struct tape temp = {0, NULL, 0, blank};
    memcpy(tape, &temp, sizeof(struct tape));
    return tape;
}

// we can only praise that this shit works
static void resize_tape(struct tape *tape, const int size) {
    if (tape != NULL && tape->ptr != NULL) {
        if (size > tape->size) {
            const int old_offset = (tape->size * OFFSET_FACTOR);
            const int relative_index = tape->index - old_offset;

            char *ptr = malloc(size * sizeof(char));
            if (ptr == NULL) {
                printf("Memory allocation failed.\n");
                exit(EXIT_FAILURE);
                return;
            }

            const int new_offset = size * OFFSET_FACTOR;
            const int offset_diff = abs(new_offset - old_offset);

            memset(ptr, tape->blank, size);
            memcpy(ptr + offset_diff, tape->ptr, tape->size);
            free(tape->ptr);

            const int new_index = new_offset + relative_index;
            tape->index = new_index;
            tape->ptr = ptr;
            tape->size = size;
        }
    }
}

void move_left(struct tape *tape) {
    if (tape != NULL) {
        if ((tape->index - 1) < 0) {
            resize_tape(tape, tape->size + 10);
        }
        tape->index--;
    }
}

void move_right(struct tape *tape) {
    if (tape != NULL) {
        if ((tape->index + 1) >= tape->size) {
            resize_tape(tape, tape->size + 10);
        }
        tape->index++;
    }
}

char read_element(const struct tape *tape) {
    if (tape != NULL && tape->ptr != NULL) {
        return *(tape->ptr + tape->index);
    }
    return 0;
}

void write_element(struct tape *tape, const char c) {
    if (tape != NULL && tape->ptr != NULL) {
        *(tape->ptr + tape->index) = c;
    }
}

void print_tape(const struct tape *tape) {
    if (tape != NULL && tape->ptr != NULL) {
        printf("%.*s", tape->index, tape->ptr);
        printf("\x1b[31m%c\x1b[0m", *(tape->ptr + tape->index));
        printf("%s", tape->ptr + tape->index + 1);
    }
}

void prepare_tape(struct tape *tape, const char *const word) {
    if (tape != NULL) {
        const int word_length = strlen(word);

        const int size = word_length + (10 - (word_length % 10));
        char *ptr = realloc(tape->ptr, size);
        if (ptr == NULL) {
            printf("Memory allocation failed.\n");
            exit(EXIT_FAILURE);
            return;
        }

        const int offset = OFFSET_FACTOR * size;
        memset(ptr, tape->blank, size);
        memcpy(ptr + offset, word, word_length);

        if (tape->ptr != NULL) {
            free(tape->ptr);
        }

        tape->index = offset;
        tape->ptr = ptr;
        tape->size = size;
    }
}

void delete_tape(struct tape *tape) {
    if (tape != NULL) {
        if (tape->ptr != NULL) {
            free(tape->ptr);
        }
        free(tape);
    }
}
