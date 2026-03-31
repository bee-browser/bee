#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Promise(u32);

base::static_assert_eq!(size_of::<Promise>(), 4);
base::static_assert_eq!(align_of::<Promise>(), 4);

impl Promise {
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }

    pub const fn as_userdata(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for Promise {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Promise> for u32 {
    fn from(value: Promise) -> Self {
        value.0
    }
}
