use std::{fmt, num::NonZeroU32};

mod sealed {
    use std::num::NonZeroU32;

    use derive_more::Display;

    /// Audio sampling rate, the number of samples in a single second (i.e.
    /// measured in hertz).
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct SampleRate(NonZeroU32);

    impl SampleRate {
        pub const fn new(n: NonZeroU32) -> Self {
            Self(n)
        }

        pub const fn get(&self) -> NonZeroU32 {
            self.0
        }
    }
}

pub use self::sealed::SampleRate;

impl fmt::Debug for SampleRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hz = self.get().get();
        if hz > 1_000 {
            let khz = hz as f32 / 1_000.;
            write!(f, "{khz:.1} kHz")
        } else {
            write!(f, "{hz} Hz")
        }
    }
}

impl From<NonZeroU32> for SampleRate {
    fn from(value: NonZeroU32) -> Self {
        Self::new(value)
    }
}

impl From<SampleRate> for NonZeroU32 {
    fn from(value: SampleRate) -> Self {
        value.get()
    }
}

#[macro_export]
macro_rules! sample_rate {
    ($hz:literal) => {
        ::audio_time::SampleRate::new(::std::num::NonZeroU32::new($hz).unwrap())
    };
}
