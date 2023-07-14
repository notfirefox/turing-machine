# Turing Machine
Proof of Concept Turing Machine

## :clipboard: Requirements
- Make

## :rocket: Usage
Use Make to compile the program.
```sh
make
```

Run the program using the following command.
```
$ build/main
Enter word: 111
q=0 |  *  * (1) 1  1  *  *
q=0 |  *  *  1 (1) 1  *  *
q=0 |  *  *  1  1 (1) *  *
q=0 |  *  *  1  1  1 (*) *
q=1 |  *  *  1  1 (1) *  *
q=1 |  *  *  1 (1) 0  *  *
q=1 |  *  * (1) 0  0  *  *
q=1 |  * (*) 0  0  0  *  *
q=2 | (*) 1  0  0  0  *  *
q=3 |  * (1) 0  0  0  *  *
```
