use std::{
    fmt,
    ops::{Div, Mul},
    time::Duration,
};

use crate::{
    convert::{
        bytes_to_frames, duration_to_frames, frames_to_bytes, frames_to_duration,
        frames_to_samples, samples_to_frames,
    },
    Bytes, Samples, System,
};

mod sealed {
    use derive_more::Display;

    use crate::System;

    /// An audio time span, measured by the number of frames contained in it.
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
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
        match frames_to_duration(self) {
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
        match duration_to_frames(dur) {
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
        match frames_to_bytes(self) {
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
        bytes_to_frames(bytes)
    }

    /// Equivalent to `Samples::try_from(frames).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_samples(self) -> Samples<SYS> {
        match frames_to_samples(self) {
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
        samples_to_frames(samples)
    }
}

impl<const SYS: System> From<usize> for Frames<SYS> {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl<const SYS: System> From<Frames<SYS>> for usize {
    #[inline]
    fn from(value: Frames<SYS>) -> Self {
        value.get()
    }
}

impl<const SYS: System> Mul for Frames<SYS> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.get().mul(rhs.get()))
    }
}

impl<const SYS: System, T> Mul<T> for Frames<SYS>
where
    usize: Mul<T, Output = usize>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.get().mul(rhs))
    }
}

impl<const SYS: System> Div for Frames<SYS> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.get().div(rhs.get()))
    }
}

impl<const SYS: System, T> Div<T> for Frames<SYS>
where
    usize: Div<T, Output = usize>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.get().div(rhs))
    }
}
