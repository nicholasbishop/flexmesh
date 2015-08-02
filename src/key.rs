use std::marker::PhantomData;

pub type NumType = u32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key<T> {
    ival: NumType,
    phantom: PhantomData<T>
}

impl<T> Key<T> {
    pub fn invalid() -> Key<T> {
        Key { ival: 0xffffffff, phantom: PhantomData }
    }

    pub fn new(val: NumType) -> Key<T> {
        Key { ival: val, phantom: PhantomData }
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
