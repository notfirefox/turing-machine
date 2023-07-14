#include <stdio.h>

#include "impl.h"
#include "turing.h"

int main() {
    char word[256];
    printf("Enter word: ");
    scanf("%255s", word);

    struct turingmachine *tm = create_turingmachine(accept1, delta1);
    process_word(tm, word);
    delete_turinmachine(tm);

    return 0;
}
