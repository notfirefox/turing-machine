#include "turing.h"

#include "tape.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct turingmachine {
    int state;
    accept_fn accept;
    delta_fn delta;
    struct tape *tape;
};

struct turingmachine *create_turingmachine(accept_fn accept, delta_fn delta) {
    struct turingmachine *tm = malloc(sizeof(struct turingmachine));
    if (tm == NULL) {
        printf("Memory allocation failed.\n");
        exit(EXIT_FAILURE);
        return NULL;
    }
    tm->state = 0;
    tm->accept = accept;
    tm->delta = delta;
    tm->tape = create_tape(BLANK);
    return tm;
}

bool process_word(struct turingmachine *tm, const char *const word) {
    prepare_tape(tm->tape, word);
    print_tape(tm->tape);
    printf(" q=%d\n", tm->state);

    while (!(*tm->accept)(tm->state)) {
        struct delta_param param = {tm->state, read_element(tm->tape)};
        struct delta_result result = (*tm->delta)(param);

        if (result.state == -1) {
            printf("Could not find path for delta(%d, %c)\n", param.state,
                   param.input);
            return false;
        }
        tm->state = result.state;
        write_element(tm->tape, result.output);

        if (result.move == right) {
            move_right(tm->tape);
        } else if (result.move == left) {
            move_left(tm->tape);
        }

        print_tape(tm->tape);
        printf(" q=%d\n", tm->state);
    }

    return true;
}

void delete_turinmachine(struct turingmachine *tm) {
    if (tm != NULL) {
        delete_tape(tm->tape);
        free(tm);
    }
}
