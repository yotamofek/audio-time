use std::{
    fmt,
    ops::{Div, Mul},
    time::Duration,
};

use crate::{Bytes, Samples, System};

mod sealed {
    use derive_more::Display;

    use crate::System;

    /// An audio time span, measured by the number of frames contained in it.
    #[derive(Copy, Eq, Hash, Display)]
    #[derive_const(Clone, PartialEq, PartialOrd, Ord)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct Frames<const SYS: System>(usize);

    impl<const SYS: System> Frames<SYS> {
        #[inline]
        pub const fn new(n: usize) -> Self {
            Self(n)
        }

        #[inline]
        pub const fn get(&self) -> usize {
            self.0
        }
    }
}

pub use self::sealed::Frames;

impl<const SYS: System> fmt::Debug for Frames<SYS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.get(), f)
    }
}

impl<const SYS: System> Frames<SYS> {
    /// Equivalent to `Duration::try_from(frames).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_duration(self) -> Duration {
        match self.try_into() {
            Ok(dur) => dur,
            Err(_) => {
                panic!("Overflowed trying to convert frames to duration")
            }
        }
    }

    /// Equivalent to `Frames::try_from(duration).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_duration(dur: Duration) -> Self {
        match dur.try_into() {
            Ok(frames) => frames,
            Err(_) => {
                panic!("Overflowed trying to convert duration to frames")
            }
        }
    }

    /// Equivalent to `Bytes::try_from(frames).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_bytes(self) -> Bytes<SYS> {
        match self.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                panic!("Overflowed trying to convert frames to duration")
            }
        }
    }

    /// Equivalent to `Frames::from(bytes)`.
    #[inline]
    #[track_caller]
    pub const fn from_bytes(bytes: Bytes<SYS>) -> Self {
        bytes.into()
    }

    /// Equivalent to `Samples::try_from(frames).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_samples(self) -> Samples<SYS> {
        match self.try_into() {
            Ok(samples) => samples,
            Err(_) => {
                panic!("Overflowed trying to convert frames to duration")
            }
        }
    }

    /// Equivalent to `Frames::from(samples)`.
    #[inline]
    #[track_caller]
    pub const fn from_samples(samples: Samples<SYS>) -> Self {
        samples.into()
    }
}

impl<const SYS: System> const From<usize> for Frames<SYS> {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl<const SYS: System> const From<Frames<SYS>> for usize {
    #[inline]
    fn from(value: Frames<SYS>) -> Self {
        value.get()
    }
}

impl<const SYS: System> const Mul for Frames<SYS> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.get().mul(rhs.get()))
    }
}

impl<const SYS: System, T> const Mul<T> for Frames<SYS>
where
    usize: ~const Mul<T, Output = usize>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.get().mul(rhs))
    }
}

impl<const SYS: System> const Div for Frames<SYS> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.get().div(rhs.get()))
    }
}

impl<const SYS: System, T> const Div<T> for Frames<SYS>
where
    usize: ~const Div<T, Output = usize>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.get().div(rhs))
    }
}
