pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
    pub pad: u8
}

impl Default for ConditionCodes {
    fn default() -> ConditionCodes {
        return ConditionCodes {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
            pad: u8::min_value(),
        };
    }
}
