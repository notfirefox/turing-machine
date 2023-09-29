#include <stdio.h>
#include <stdlib.h>

#include "impl.h"
#include "turing.h"

int main() {
    char *word = NULL;
    printf("Enter word: ");
    scanf("%ms", &word);

    struct turingmachine *tm = create_turingmachine(accept1, delta1);
    process_word(tm, word);
    delete_turinmachine(tm);

    free(word);
    return 0;
}
