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
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: vec![0; 65536],
            cc: Default::default(),
            int_enable: u8::min_value()
        };
    }
}

impl State {
    pub fn print(&self) {
        println!("State");
        println!("=================");
        println!("af: {:02X}", self.a);
        println!("bc: {:02X}{:02X}", self.b, self.c);
        println!("de: {:02X}{:02X}", self.d, self.e);
        println!("hl: {:02X}{:02X}", self.h, self.l);
        println!("pc: {:04X}", self.pc);
        println!("sp: {:04X}", self.sp);
        self.cc.print();
    }
}
