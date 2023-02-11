//! Implementations of the [`From`] and [`TryFrom`] traits for converting
//! between [`Samples`], [`Bytes`], and [`Duration`]s.

use std::time::Duration;

use crate::{Bytes, OverflowError, Samples, System};

//
// Samples <-> Bytes
//

impl<const SYS: System> const From<Bytes<SYS>> for Samples<SYS> {
    #[inline]
    fn from(value: Bytes<SYS>) -> Self {
        Self::new(value.get() / usize::from(SYS.sample_size().get()))
    }
}

impl<const SYS: System> const TryFrom<Samples<SYS>> for Bytes<SYS> {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
        let bytes = value.get().checked_mul(SYS.sample_size().get().into());

        match bytes {
            Some(n) => Ok(Self::new(n).unwrap()),
            None => Err(OverflowError(())),
        }
    }
}

//
// Samples <-> Duration
//

impl<const SYS: System> const TryFrom<Duration> for Samples<SYS> {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let samples: Option<usize> = try {
            let sample_rate = SYS.sample_rate.get().get().into();
            let samples = value.as_millis().checked_mul(sample_rate)? / 1_000;

            samples.try_into().ok()?
        };

        match samples {
            Some(n) => Ok(Self::new(n)),
            None => Err(OverflowError(())),
        }
    }
}

impl<const SYS: System> const TryFrom<Samples<SYS>> for Duration {
    type Error = OverflowError;

    #[inline]
    fn try_from(value: Samples<SYS>) -> Result<Self, Self::Error> {
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
