use cpu::condition_codes::ConditionCodes;

pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8
}

impl Default for State {
    fn default() -> State {
        return State {
            a: u8::min_value(),
            b: u8::min_value(),
            c: u8::min_value(),
            d: u8::min_value(),
            e: u8::min_value(),
            h: u8::min_value(),
            l: u8::min_value(),
            sp: u16::min_value(),
            pc: u16::min_value(),
            memory: Vec::new(),
            cc: Default::default(),
            int_enable: u8::min_value()
        };
    }
}
