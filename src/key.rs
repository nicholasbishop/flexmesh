pub type NumType = u32;

#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub struct Key {
    ival: NumType
}

impl Key {
    pub fn invalid() -> Key {
        Key { ival: 0xffffffff }
    }

    pub fn new(val: NumType) -> Key {
        Key { ival: val }
    }

    pub fn valid(&self) -> bool {
        self.ival == 0xffffffff
    }

    pub fn value(&self) -> NumType {
        if self.valid() {
            self.ival
        } else {
            panic!("invalid key");
        }
    }
}
