#include "tape.h"

#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct tape {
    int index;
    int dirty;
    char *ptr;
    size_t size;
    const char blank;
};

struct tape *create_tape(const char blank) {
    struct tape *tape = malloc(sizeof(struct tape));
    if (tape == NULL) {
        printf("Memory allocation failed.\n");
        exit(EXIT_FAILURE);
        return NULL;
    }
    const struct tape temp = {0, INT_MAX, NULL, 0, blank};
    memcpy(tape, &temp, sizeof(struct tape));
    return tape;
}

static void extend_left(struct tape *tape) {
    if (tape != NULL && tape->ptr != NULL) {
        char *ptr = malloc((tape->size + 1) * sizeof(char));
        if (ptr == NULL) {
            printf("Memory allocation failed.\n");
            exit(EXIT_FAILURE);
            return;
        }
        *ptr = tape->blank;
        memcpy(ptr + 1, tape->ptr, tape->size);
        free(tape->ptr);

        tape->ptr = ptr;
        tape->index++;
        tape->size++;
    }
}

static void extend_right(struct tape *tape) {
    if (tape != NULL && tape->ptr != NULL) {
        char *ptr = malloc((tape->size + 1) * sizeof(char));
        if (ptr == NULL) {
            printf("Memory allocation failed.\n");
            exit(EXIT_FAILURE);
            return;
        }
        *(tape->ptr + tape->size) = tape->blank;
        memcpy(ptr, tape->ptr, tape->size);
        free(tape->ptr);

        tape->ptr = ptr;
        tape->index--;
        tape->size++;
    }
}

void move_left(struct tape *tape) {
    if (tape != NULL) {
        if ((tape->index - 1) < 0) {
            extend_left(tape);
        }
        tape->index--;
    }
}

void move_right(struct tape *tape) {
    if (tape != NULL) {
        if ((tape->index + 1) >= tape->size) {
            extend_right(tape);
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
        if (*(tape->ptr + tape->index) != c) {
            *(tape->ptr + tape->index) = c;
            tape->dirty = tape->index;
        }
    }
}

void reset_dirty(struct tape *tape) {
    if (tape != NULL) {
        tape->dirty = INT_MAX;
    }
}

void print_tape(const struct tape *tape) {
    if (tape != NULL && tape->ptr != NULL) {
        for (int i = 0; i < tape->size; i++) {
            char current = *(tape->ptr + i);
            bool is_index = i == tape->index;
            printf("%s", is_index ? "\x1b[34m(\x1b[0m" : " ");
            if (i == tape->dirty) {
                printf("\x1b[33m%c\x1b[0m", current);
            } else {
                printf("%c", current);
            }
            printf("%s", is_index ? "\x1b[34m)\x1b[0m" : " ");
        }
        printf("\n");
    }
}

void prepare_tape(struct tape *tape, const char *const word) {
    if (tape != NULL && word != NULL) {
        const size_t word_length = strlen(word);

        const size_t size = word_length + 4;
        char *ptr = realloc(tape->ptr, size);
        if (ptr == NULL) {
            printf("Memory allocation failed.\n");
            exit(EXIT_FAILURE);
            return;
        }

        const int offset = 2;
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
