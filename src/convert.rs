//! Implementations of the [`From`] and [`TryFrom`] traits for converting
//! between [`Samples`], [`Bytes`], and [`Duration`]s.

use std::time::Duration;

pub(crate) use self::{frames::*, samples::*};
use crate::{frames::Frames, Bytes, OverflowError, Samples, System};

mod frames {
    use super::*;

    //
    // Frames <-> Bytes
    //

    pub(crate) const fn bytes_to_frames<const SYS: System>(value: Bytes<SYS>) -> Frames<SYS> {
        Frames::new(value.get() / SYS.frame_size().get() as usize)
    }

    pub(crate) const fn frames_to_bytes<const SYS: System>(
        value: Frames<SYS>,
    ) -> Result<Bytes<SYS>, OverflowError> {
        let bytes = value.get().checked_mul(SYS.frame_size().get() as usize);

        match bytes {
            Some(n) => Ok(Bytes::new(n).unwrap()),
            None => Err(OverflowError(())),
        }
    }

    impl<const SYS: System> From<Bytes<SYS>> for Frames<SYS> {
        #[inline]
        fn from(value: Bytes<SYS>) -> Self {
            bytes_to_frames(value)
        }
    }

    impl<const SYS: System> TryFrom<Frames<SYS>> for Bytes<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            frames_to_bytes(value)
        }
    }

    //
    // Frames <-> Samples
    //

    pub(crate) const fn samples_to_frames<const SYS: System>(value: Samples<SYS>) -> Frames<SYS> {
        Frames::new(value.get() / SYS.channel_layout.channels().get() as usize)
    }

    pub(crate) const fn frames_to_samples<const SYS: System>(
        value: Frames<SYS>,
    ) -> Result<Samples<SYS>, OverflowError> {
        let bytes = value
            .get()
            .checked_mul(SYS.channel_layout.channels().get() as usize);

        match bytes {
            Some(n) => Ok(Samples::new(n).unwrap()),
            None => Err(OverflowError(())),
        }
    }

    impl<const SYS: System> From<Samples<SYS>> for Frames<SYS> {
        #[inline]
        fn from(value: Samples<SYS>) -> Self {
            samples_to_frames(value)
        }
    }

    impl<const SYS: System> TryFrom<Frames<SYS>> for Samples<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            frames_to_samples(value)
        }
    }

    //
    // Frames <-> Duration
    //

    #[cfg(target_pointer_width = "64")] // TODO: impl for other archs
    pub(crate) const fn duration_to_frames<const SYS: System>(
        value: Duration,
    ) -> Result<Frames<SYS>, OverflowError> {
        let sample_rate = SYS.sample_rate.get().get() as u128;
        let frames = match value.as_millis().checked_mul(sample_rate) {
            Some(frames) => Some(frames / 1_000),
            None => None,
        };

        match frames {
            Some(n) if n <= usize::MAX as u128 => Ok(Frames::new(n as usize)),
            _ => Err(OverflowError(())),
        }
    }

    #[cfg(target_pointer_width = "64")] // TODO: impl for other archs
    pub(crate) const fn frames_to_duration<const SYS: System>(
        value: Frames<SYS>,
    ) -> Result<Duration, OverflowError> {
        let sample_rate = SYS.sample_rate.get().get() as u64;

        let millis = match value.get().checked_mul(1_000) {
            Some(n) => Some(n as u64 / sample_rate),
            None => None,
        };

        match millis {
            Some(n) => Ok(Duration::from_millis(n)),
            None => Err(OverflowError(())),
        }
    }

    impl<const SYS: System> TryFrom<Duration> for Frames<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Duration) -> Result<Self, Self::Error> {
            duration_to_frames(value)
        }
    }

    impl<const SYS: System> TryFrom<Frames<SYS>> for Duration {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            frames_to_duration(value)
        }
    }
}

mod samples {
    use super::*;

    //
    // Samples <-> Bytes
    //

    pub(crate) const fn bytes_to_samples<const SYS: System>(value: Bytes<SYS>) -> Samples<SYS> {
        Samples::new(value.get() / SYS.sample_type.byte_depth().get() as usize).unwrap()
    }

    pub(crate) const fn samples_to_bytes<const SYS: System>(
        value: Samples<SYS>,
    ) -> Result<Bytes<SYS>, OverflowError> {
        let bytes = value
            .get()
            .checked_mul(SYS.sample_type.byte_depth().get() as usize);

        match bytes {
            Some(n) => Ok(Bytes::new(n).unwrap()),
            None => Err(OverflowError(())),
        }
    }

    impl<const SYS: System> From<Bytes<SYS>> for Samples<SYS> {
        #[inline]
        fn from(value: Bytes<SYS>) -> Self {
            bytes_to_samples(value)
        }
    }

    impl<const SYS: System> TryFrom<Samples<SYS>> for Bytes<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
            samples_to_bytes(value)
        }
    }

    //
    // Samples <-> Duration (via Frames)
    //

    impl<const SYS: System> TryFrom<Duration> for Samples<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Duration) -> Result<Self, Self::Error> {
            let frames = Frames::try_from(value)?;
            frames.try_into()
        }
    }

    impl<const SYS: System> TryFrom<Samples<SYS>> for Duration {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
            let frames = Frames::from(value);
            frames.try_into()
        }
    }
}

//
// Bytes <-> Duration (via Samples)
//

impl<const SYS: System> TryFrom<Duration> for Bytes<SYS> {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Samples::<SYS>::try_from(value)?.try_into()
    }
}

impl<const SYS: System> TryFrom<Bytes<SYS>> for Duration {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Bytes<SYS>) -> Result<Self, Self::Error> {
        Samples::<SYS>::from(value).try_into()
    }
}
