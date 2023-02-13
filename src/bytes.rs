use std::{fmt, time::Duration};

use crate::{Samples, System};

mod sealed {
    use derive_more::Display;

    use crate::System;

    /// An audio time span, measured in the number of bytes required for its
    /// representation.
    ///
    /// The `usize` contained in this struct is invariantly held to be divisible
    /// (without remainder) by the size of a single sample
    /// ([`SYS.sample_size()`](System::sample_size)).
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct Bytes<const SYS: System>(usize);

    impl<const SYS: System> Bytes<SYS> {
        /// Create a `Bytes` if the given value is divisible by
        /// [`SYS.sample_size()`](System::sample_size).
        #[inline]
        pub const fn new(n: usize) -> Option<Self> {
            let rem = n % usize::from(SYS.sample_size().get());

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

impl<const SYS: System> fmt::Debug for Bytes<SYS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.get(), f)
    }
}

impl<const SYS: System> Bytes<SYS> {
    /// Equivalent to `Duration::try_from(bytes).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_duration(self) -> Duration {
        match self.try_into() {
            Ok(dur) => dur,
            Err(_) => {
                panic!("Overflowed trying to convert bytes to duration")
            }
        }
    }

    /// Equivalent to `Bytes::try_from(duration).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_duration(dur: Duration) -> Self {
        match dur.try_into() {
            Ok(samples) => samples,
            Err(_) => {
                panic!("Overflowed trying to convert duration to bytes")
            }
        }
    }

    /// Equivalent to `Samples::from(bytes)`.
    #[inline]
    #[track_caller]
    pub const fn into_samples(self) -> Samples<SYS> {
        self.into()
    }

    /// Equivalent to `Bytes::try_from(samples).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_samples(samples: Samples<SYS>) -> Self {
        match samples.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                panic!("Overflowed trying to convert samples to bytes")
            }
        }
    }
}

impl<const SYS: System> const From<Bytes<SYS>> for usize {
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
