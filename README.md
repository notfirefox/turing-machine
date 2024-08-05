# Turing Machine
Proof of Concept Turing Machine

> [!NOTE]
> The program has been rewritten in Rust. The original version can be found 
> [here](https://github.com/notfirefox/turing-machine/tree/e871b32629996e33f8fded3f4b042aab9f65f407).

## :clipboard: Requirements 
- Rust

## :rocket: Usage
Run the program using the following command.
```sh
cargo run
```

Lets consider what happens for the input `111`:
```
$ cargo run
Enter word: 
111
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
