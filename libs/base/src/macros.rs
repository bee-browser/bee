#[macro_export]
macro_rules! delegate_all {
    ($type:ty => $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($type:ty => $target:ty, $lifetime:lifetime) => {
        impl<$lifetime> std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$lifetime> std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub use delegate_all;
