use cpu::state::State;

pub struct CPU {
    state: State
}

impl CPU {
    pub fn emulate_op(&self) {
        println!("asdf");
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.state.memory = rom;
    }
}

impl Default for CPU {
    fn default() -> CPU {
        return CPU {
            state: Default::default()
        };
    }
}
