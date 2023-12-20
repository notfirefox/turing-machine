pub const BLANK : char = '_';

const OFFSET_FACTOR : f32 = 0.2;

pub struct DeltaParam {
    pub state: i32,
    pub input: char,
}

#[derive(PartialEq)]
pub enum Move {
    Right, Left, None
}

pub struct DeltaResult {
    state: i32,
    output: char,
    r#move: Move,
}

impl DeltaResult {
    pub fn new(state: i32, output: char, r#move: Move) -> DeltaResult {
        DeltaResult { state: state, output: output, r#move: r#move }
    }
}

pub type AcceptFn = Box<dyn Fn(i32) -> bool>;
pub type DeltaFn = Box<dyn Fn(&DeltaParam) -> DeltaResult>;

pub struct Tape {
    index: i32,
    vector: Vec<char>,
    blank: char,
}

impl Tape {
    fn new() -> Tape {
        Tape { index: 0, vector: vec![], blank: BLANK }
    }

    fn resize(&mut self, size: usize) {
        let old_offset = ((self.vector.len() as f32) * OFFSET_FACTOR) as i32;
        let relative_index = self.index - old_offset;

        let new_offset = ((size as f32) * OFFSET_FACTOR) as i32;
        let offset_diff = (new_offset - old_offset).abs();
        let new_index = new_offset + relative_index;

        // clone the content of the vector into `copy`
        let copy = self.vector.clone();

        // set up new vector
        self.vector = vec![self.blank; size];
        for i in 0..copy.len() {
            self.vector[((offset_diff as usize) + i) as usize] = copy[i];
        }

        // assign the index to the new index
        self.index = new_index;
    }

    fn move_left(&mut self) {
        if (self.index - 1) < 0 {
            self.resize(self.vector.len() + 10);
        }
        self.index -= 1;
    }

    fn move_right(&mut self) {
        if (self.index + 1) >= (self.vector.len() as i32) {
            self.resize(self.vector.len() + 10);
        }
        self.index += 1;
    }

    fn read_element(&self) -> char {
        self.vector[self.index as usize]
    }

    fn write_element(&mut self, c: char) {
        self.vector[self.index as usize] = c;
    }

    fn prepare(&mut self, word: &String) {
        let word_length = word.len();

        // Create a new vector and load blanks into it
        let size = word_length + (10 - (word_length % 10));
        let mut new_vector = vec![self.blank; size];

        let offset = (OFFSET_FACTOR * (size as f32)) as usize;

        // Load the word into the new vector
        let mut i = 0;
        for c in word.chars() {
            new_vector[offset + i] = c;
            i += 1;
        }

        // Replace the old vector with the new one
        self.vector = new_vector;
        self.index = offset as i32;
    }

    fn print(&self) {
        for i in 0..self.vector.len() {
            if i == (self.index as usize) {
                print!("\x1b[31m{}\x1b[0m", self.vector[i]);
            } else {
                print!("{}", self.vector[i]);
            }
        }
    }
}

pub struct TuringMachine {
    state: i32,
    accept_fn: AcceptFn,
    delta_fn: DeltaFn,
    tape: Tape,
}

impl TuringMachine {
    pub fn new(accept : AcceptFn, delta: DeltaFn) -> TuringMachine {
        TuringMachine {
            state: 0,
            accept_fn: accept,
            delta_fn: delta,
            tape: Tape::new(),
        }
    }
     
    pub fn process_word(&mut self, word: &String) -> bool {
        self.tape.prepare(word);
        self.tape.print();
        println!(" q={}", self.state);

        let accept_fn = &self.accept_fn;
        let delta_fn = &self.delta_fn;

        while !accept_fn(self.state) {
            let param = DeltaParam {
                state: self.state,
                input: self.tape.read_element(),
            };
            let result = delta_fn(&param);

            if result.state == -1 {
                println!("Could not find path for delta({}, {})", &param.state, &param.input);
                return false;
            }

            self.state = result.state;
            self.tape.write_element(result.output);

            if result.r#move == Move::Left {
                self.tape.move_left();
            } else if result.r#move == Move::Right {
                self.tape.move_right();
            }

            self.tape.print();
            println!(" q={}", self.state);
        }

        true
    }
}
