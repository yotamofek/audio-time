//! Encode and convert audio time spans between representations in number of
//! samples, number of bytes and time duration.
//!
//! ```rust
//! # #![feature(const_option)]
//! # use std::time::Duration;
//! # use audio_time::*;
//! #
//! // the Audio CD standard defines the encoding system as follows:
//! // 2 channels of LPCM audio, each signed 16-bit values sampled at 44100 Hz
//! let frames = Frames::<AUDIO_CD>::from_duration(Duration::from_secs(1));
//! assert_eq!(44_100, frames.get());
//! let samples = frames.into_samples();
//! assert_eq!(88_200, samples.get());
//! let bytes = samples.into_bytes();
//! assert_eq!(176_400, bytes.get());
//!
//! // both `Samples` and `Bytes` can be converted back into `Duration`s:
//! assert_eq!(bytes.into_duration(), Duration::from_secs(1));
//! assert_eq!((samples * 2).into_duration(), Duration::from_secs(2));
//!
//! // let's define our own `System`
//! const SYSTEM: System = system!(8_000, Mono, i16);
//! let frames = Frames::<SYSTEM>::from_duration(Duration::from_secs(1));
//! assert_eq!(8_000, frames.get());
//! // in `Mono` systems, `Frames` and `Samples` are always equal
//! assert_eq!(8_000, frames.into_samples().get());
//! let bytes = frames.into_bytes();
//! assert_eq!(16_000, bytes.get());
//! ```

#![allow(incomplete_features)]
#![feature(
    adt_const_params,
    const_trait_impl,
    const_try,
    derive_const,
    try_blocks
)]

extern crate self as audio_time;

mod bytes;
mod channel_layout;
mod convert;
mod frames;
mod macros;
mod sample;
mod sample_rate;
mod samples;
mod system;

pub use ChannelLayout::{Mono, Stereo};

pub use crate::{
    bytes::Bytes,
    channel_layout::ChannelLayout,
    frames::Frames,
    sample::SampleType,
    sample_rate::SampleRate,
    samples::Samples,
    system::{System, AUDIO_CD},
};

#[derive(thiserror::Error, Debug)]
#[error("Overflow error")]
pub struct OverflowError(());

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use audio_time::*;

    macro_rules! assert_bidi {
        ($a:expr, $b:expr) => {
            assert_eq!($a, $b.try_into().unwrap());
            assert_eq!($b, $a.try_into().unwrap());
        };
    }

    #[test]
    fn test_frames_to_duration() -> Result<(), OverflowError> {
        assert_bidi!(
            Duration::ZERO,
            Frames::<{ system!(44_100, Mono, i16) }>::new(0)
        );

        assert_bidi!(
            Duration::from_secs(2),
            Frames::<{ system!(44_100, Mono, i16) }>::new(88_200)
        );

        assert_bidi!(
            Duration::from_millis(100),
            Frames::<{ system!(8_000, Mono, i16) }>::new(800)
        );

        // sample type should not matter in this conversion
        assert_bidi!(
            Duration::from_millis(100),
            Frames::<{ system!(8_000, Mono, f64) }>::new(800)
        );

        // test overflow
        assert!(
            Duration::try_from(Frames::<{ system!(8_000, Mono, i16) }>::new(usize::MAX)).is_err()
        );
        assert!(Frames::<{ system!(8_000, Mono, i16) }>::try_from(Duration::MAX).is_err());

        {
            const SYS: System = system!(48_000, Mono, i16);
            let millisecond = Frames::<SYS>::new(48);
            assert_eq!(Duration::from_millis(1), millisecond.try_into()?);

            let sub_millisecond = Frames::<SYS>::new(millisecond.get() - 1);
            // this conversion is lossy for durations of under 1 milliseconds
            assert_eq!(Duration::from_millis(0), sub_millisecond.try_into()?);
            assert_ne!(
                sub_millisecond,
                Duration::try_from(sub_millisecond)?.try_into()?
            );
        }

        Ok(())
    }

    #[test]
    fn test_frames_to_bytes() -> Result<(), OverflowError> {
        assert_bidi!(bytes!(0), Frames::<{ system!(44_100, Mono, i16) }>::new(0));

        assert_bidi!(
            bytes!(2_000),
            Frames::<{ system!(48_000, Mono, i16) }>::new(1_000)
        );

        assert_bidi!(
            bytes!(4_000),
            Frames::<{ system!(48_000, Stereo, i16) }>::new(1_000)
        );

        assert_bidi!(
            bytes!(8_000),
            Frames::<{ system!(48_000, Stereo, i32) }>::new(1_000)
        );

        assert_bidi!(
            bytes!(16_000),
            Frames::<{ system!(48_000, Stereo, i32) }>::new(2_000)
        );

        // sample rate should not matter in this conversion
        assert_bidi!(
            bytes!(8_000),
            Frames::<{ system!(8_000, Stereo, i32) }>::new(1_000)
        );

        Ok(())
    }
}
