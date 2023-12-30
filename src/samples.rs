use std::{ops::Mul, time::Duration};

use crate::{
    convert::{bytes_to_samples, frames_to_samples, samples_to_bytes, samples_to_frames},
    impl_fmt, Bytes, Frames, System,
};

mod sealed {
    use crate::System;

    /// An audio time span, measured by the number of samples contained in it.
    ///
    ///
    /// The `usize` contained in this struct is invariantly held to be divisible
    /// (without remainder) by the number of channels in the system
    /// ([`SYS.channel_layout.channels()`](crate::ChannelLayout::channels)).

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct Samples<const SYS: System>(usize);

    impl<const SYS: System> Samples<SYS> {
        /// Create a `Samples` if the given value is divisible by
        /// [`SYS.channel_layout.channels()`](crate::ChannelLayout::channels).
        #[inline]
        pub const fn new(n: usize) -> Option<Self> {
            let rem = n % SYS.channel_layout.channels().get() as usize;

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

pub use self::sealed::Samples;

impl_fmt!(Samples);

impl<const SYS: System> Samples<SYS> {
    /// Equivalent to `Duration::try_from(samples).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_duration(self) -> Duration {
        self.into_frames().into_duration()
    }

    /// Equivalent to `Samples::try_from(duration).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_duration(dur: Duration) -> Self {
        Self::from_frames(Frames::from_duration(dur))
    }

    /// Equivalent to `Bytes::try_from(samples).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn into_bytes(self) -> Bytes<SYS> {
        match samples_to_bytes(self) {
            Ok(bytes) => bytes,
            Err(_) => {
                panic!("Overflowed trying to convert samples to duration")
            }
        }
    }

    /// Equivalent to `Samples::from(bytes)`.
    #[inline]
    #[track_caller]
    pub const fn from_bytes(bytes: Bytes<SYS>) -> Self {
        bytes_to_samples(bytes)
    }

    /// Equivalent to `Frames::from(samples)`.
    #[inline]
    #[track_caller]
    pub const fn into_frames(self) -> Frames<SYS> {
        samples_to_frames(self)
    }

    /// Equivalent to `Samples::try_from(frames).unwrap()`.
    #[inline]
    #[track_caller]
    pub const fn from_frames(frames: Frames<SYS>) -> Self {
        match frames_to_samples(frames) {
            Ok(samples) => samples,
            Err(_) => {
                panic!("Overflowed trying to convert frames to samples")
            }
        }
    }
}

impl<const SYS: System> From<Samples<SYS>> for usize {
    #[inline]
    fn from(value: Samples<SYS>) -> Self {
        value.get()
    }
}

impl<const SYS: System> Mul for Samples<SYS> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.get().mul(rhs.get())).unwrap()
    }
}

impl<const SYS: System, T> Mul<T> for Samples<SYS>
where
    usize: Mul<T, Output = usize>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.get().mul(rhs)).unwrap()
    }
}
