use std::time::Duration;

use crate::{
    convert::{bytes_to_samples, samples_to_bytes},
    impl_fmt, Samples, System,
};

mod sealed {
    use crate::System;

    /// An audio time span, measured in the number of bytes required for its
    /// representation.
    ///
    /// The `usize` contained in this struct is invariantly held to be divisible
    /// (without remainder) by the size of a single frame
    /// ([`SYS.frame_size()`](System::frame_size)).
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct Bytes<const SYS: System>(usize);

    impl<const SYS: System> Bytes<SYS> {
        /// Create a `Bytes` if the given value is divisible by
        /// [`SYS.frame_size()`](System::frame_size).
        #[inline]
        pub const fn new(n: usize) -> Option<Self> {
            let rem = n % SYS.frame_size().get() as usize;

            if rem == 0 {
                Some(Self(n))
            } else {
                None
            }
        }

        #[inline]
        pub const fn get(&self) -> usize {
            self.0
        }
    }
}

pub use self::sealed::Bytes;

impl_fmt!(Bytes);

impl<const SYS: System> Bytes<SYS> {
    /// Equivalent to `Duration::try_from(bytes).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_duration(self) -> Duration {
        self.into_samples().into_duration()
    }

    /// Equivalent to `Bytes::try_from(duration).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_duration(dur: Duration) -> Self {
        Self::from_samples(Samples::from_duration(dur))
    }

    /// Equivalent to `Samples::from(bytes)`.
    #[inline]
    #[track_caller]
    pub const fn into_samples(self) -> Samples<SYS> {
        bytes_to_samples(self)
    }

    /// Equivalent to `Bytes::try_from(samples).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_samples(samples: Samples<SYS>) -> Self {
        match samples_to_bytes(samples) {
            Ok(bytes) => bytes,
            Err(_) => {
                panic!("Overflowed trying to convert samples to bytes")
            }
        }
    }
}

impl<const SYS: System> From<Bytes<SYS>> for usize {
    #[inline]
    fn from(value: Bytes<SYS>) -> Self {
        value.get()
    }
}

#[macro_export]
macro_rules! bytes {
    ($n:literal) => {
        ::audio_time::Bytes::new($n).unwrap()
    };
}
