# Turing Machine
Proof of Concept Turing Machine

## :clipboard: Requirements
- CMake >= 3.20

## :rocket: Usage
Use CMake to compile the program.
```sh
cmake -B build/ && cmake --build build/
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
