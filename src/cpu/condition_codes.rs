pub struct ConditionCodes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8
}

impl Default for ConditionCodes {
    fn default() -> ConditionCodes {
        return ConditionCodes {
            z: u8::min_value(),
            s: u8::min_value(),
            p: u8::min_value(),
            cy: u8::min_value(),
            ac: u8::min_value(),
            pad: u8::min_value(),
        };
    }
}
