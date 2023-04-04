//! Implementations of the [`From`] and [`TryFrom`] traits for converting
//! between [`Samples`], [`Bytes`], and [`Duration`]s.

use std::time::Duration;

use crate::{frames::Frames, Bytes, OverflowError, Samples, System};

mod frames {
    use super::*;

    //
    // Frames <-> Bytes
    //

    impl<const SYS: System> const From<Bytes<SYS>> for Frames<SYS> {
        #[inline]
        fn from(value: Bytes<SYS>) -> Self {
            Self::new(value.get() / usize::from(SYS.frame_size().get()))
        }
    }

    impl<const SYS: System> const TryFrom<Frames<SYS>> for Bytes<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            let bytes = value.get().checked_mul(SYS.frame_size().get().into());

            match bytes {
                Some(n) => Ok(Self::new(n).unwrap()),
                None => Err(OverflowError(())),
            }
        }
    }

    //
    // Frames <-> Samples
    //

    impl<const SYS: System> const From<Samples<SYS>> for Frames<SYS> {
        #[inline]
        fn from(value: Samples<SYS>) -> Self {
            Self::new(value.get() / usize::from(SYS.channel_layout.channels().get()))
        }
    }

    impl<const SYS: System> const TryFrom<Frames<SYS>> for Samples<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            let bytes = value
                .get()
                .checked_mul(SYS.channel_layout.channels().get().into());

            match bytes {
                Some(n) => Ok(Self::new(n).unwrap()),
                None => Err(OverflowError(())),
            }
        }
    }

    //
    // Frames <-> Duration
    //

    impl<const SYS: System> const TryFrom<Duration> for Frames<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Duration) -> Result<Self, Self::Error> {
            let frames: Option<usize> = try {
                let sample_rate = SYS.sample_rate.get().get().into();
                let frames = value.as_millis().checked_mul(sample_rate)? / 1_000;

                frames.try_into().ok()?
            };

            match frames {
                Some(n) => Ok(Self::new(n)),
                None => Err(OverflowError(())),
            }
        }
    }

    impl<const SYS: System> const TryFrom<Frames<SYS>> for Duration {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Frames<SYS>) -> Result<Self, Self::Error> {
            let millis: Option<u64> = try {
                let sample_rate = usize::try_from(SYS.sample_rate.get().get()).ok()?;
                let millis = value.get().checked_mul(1_000)? / sample_rate;
                millis.try_into().ok()?
            };

            match millis {
                Some(n) => Ok(Duration::from_millis(n)),
                None => Err(OverflowError(())),
            }
        }
    }
}

mod samples {
    use super::*;

    //
    // Samples <-> Bytes
    //

    impl<const SYS: System> const From<Bytes<SYS>> for Samples<SYS> {
        #[inline]
        fn from(value: Bytes<SYS>) -> Self {
            Self::new(value.get() / usize::from(SYS.sample_type.byte_depth().get())).unwrap()
        }
    }

    impl<const SYS: System> const TryFrom<Samples<SYS>> for Bytes<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
            let bytes = value
                .get()
                .checked_mul(SYS.sample_type.byte_depth().get().into());

            match bytes {
                Some(n) => Ok(Self::new(n).unwrap()),
                None => Err(OverflowError(())),
            }
        }
    }

    //
    // Samples <-> Duration (via Frames)
    //

    impl<const SYS: System> const TryFrom<Duration> for Samples<SYS> {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Duration) -> Result<Self, Self::Error> {
            let frames = Frames::try_from(value)?;
            frames.try_into()
        }
    }

    impl<const SYS: System> const TryFrom<Samples<SYS>> for Duration {
        type Error = OverflowError;

        #[inline]
        fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
            let frames = Frames::from(value);
            frames.try_into()
        }
    }
}

//
// Bytes <-> Duration
//

impl<const SYS: System> const TryFrom<Duration> for Bytes<SYS> {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Samples::<SYS>::try_from(value)?.try_into()
    }
}

impl<const SYS: System> const TryFrom<Bytes<SYS>> for Duration {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Bytes<SYS>) -> Result<Self, Self::Error> {
        Samples::<SYS>::from(value).try_into()
    }
}
