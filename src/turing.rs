use core::fmt;

pub const BLANK: char = '*';

#[derive(PartialEq, Eq)]
pub enum Move {
    Right,
    Left,
    None,
}

pub struct DeltaResult {
    state: usize,
    output: char,
    r#move: Move,
}

impl DeltaResult {
    #[must_use]
    pub const fn new(state: usize, output: char, r#move: Move) -> Self {
        Self {
            state,
            output,
            r#move,
        }
    }
}

pub type AcceptFn = Box<dyn Fn(usize) -> bool>;
pub type DeltaFn = Box<dyn Fn(usize, char) -> Option<DeltaResult>>;

pub struct Tape {
    index: usize,
    dirty: usize,
    vector: Vec<char>,
}

impl Tape {
    const fn new() -> Self {
        Self {
            index: 0,
            dirty: usize::MAX,
            vector: vec![],
        }
    }

    fn move_left(&mut self) {
        if self.index == 0 {
            // insert one blank symbol on the left
            // by shifting all elements to the right
            self.vector.insert(0, BLANK);
            self.index += 1;
        }
        self.index -= 1;
    }

    fn move_right(&mut self) {
        if self.index == (self.vector.len() - 1) {
            // append a blank symbol to the vector so that
            // we can safely increment the element index
            self.vector.push(BLANK);
        }
        self.index += 1;
    }

    fn read_element(&self) -> char {
        self.vector[self.index]
    }

    fn write_element(&mut self, c: char) {
        if self.vector[self.index] != c {
            self.vector[self.index] = c;
            self.dirty = self.index;
        }
    }

    fn reset_dirty(&mut self) {
        self.dirty = usize::MAX;
    }

    fn prepare(&mut self, word: &str) {
        self.vector = word.chars().collect();
        self.vector.insert(0, BLANK);
        self.vector.insert(0, BLANK);
        self.vector.push(BLANK);
        self.vector.push(BLANK);
        self.index = 2;
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self
            .vector
            .iter()
            .enumerate()
            .map(|(i, v)| match i {
                i if i == self.index => {
                    format!("\x1b[34m(\x1b[0m{}\x1b[34m)\x1b[0m", self.vector[i])
                }
                i if i == self.dirty => {
                    format!(" \x1b[33m{}\x1b[0m ", self.vector[i])
                }
                _ => format!(" {v} "),
            })
            .collect();
        let joined = strings.join("");
        write!(f, "{joined}")
    }
}

pub struct Machine {
    state: usize,
    accept_fn: AcceptFn,
    delta_fn: DeltaFn,
    tape: Tape,
}

impl Machine {
    #[must_use]
    pub fn new(accept: AcceptFn, delta: DeltaFn) -> Self {
        Self {
            state: 0,
            accept_fn: accept,
            delta_fn: delta,
            tape: Tape::new(),
        }
    }

    pub fn process_word(&mut self, word: &str) -> bool {
        self.tape.prepare(word);
        println!("{self}");

        loop {
            let state = self.state;
            let input = self.tape.read_element();

            if let Some(result) = (self.delta_fn)(state, input) {
                self.state = result.state;
                self.tape.write_element(result.output);

                match result.r#move {
                    Move::Left => self.tape.move_left(),
                    Move::Right => self.tape.move_right(),
                    Move::None => (),
                };

                println!("{self}");
                self.tape.reset_dirty();
            } else {
                break;
            }
        }

        (self.accept_fn)(self.state)
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tape = format!("{}", self.tape);
        write!(f, "q={} | {}", self.state, tape)
    }
}
