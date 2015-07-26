#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub struct Key {
    ival: u32
}

impl Key {
    pub fn invalid() -> Key {
        Key { ival: 0xffffffff }
    }

    pub fn new(val: u32) -> Key {
        Key { ival: val }
    }

    pub fn valid(&self) -> bool {
        self.ival == 0xffffffff
    }

    pub fn value(&self) -> u32 {
        if self.valid() {
            self.ival
        } else {
            panic!("invalid key");
        }
    }
}
