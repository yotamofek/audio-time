use std::{marker::ConstParamTy, num::NonZeroU8};

use crate::{ChannelLayout, SampleRate, SampleType};

/// A struct that encodes all parameters that are needed to interpret an audio
/// time span as number of samples and/or the number of bytes needed to
/// represent it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
pub struct System {
    pub sample_rate: SampleRate,
    pub channel_layout: ChannelLayout,
    pub sample_type: SampleType,
}

impl System {
    /// The number of bytes used to represent a single frame.
    ///
    /// Equal to the sample's byte depth
    /// ([`self.sample_type.byte_depth()`](crate::SampleType::byte_depth)) times
    /// the number of channels
    /// ([`self.channel_layout.channels()`](crate::ChannelLayout::channels)).
    #[inline]
    #[track_caller]
    pub const fn frame_size(&self) -> NonZeroU8 {
        self.channel_layout
            .channels()
            .checked_mul(self.sample_type.byte_depth())
            .expect("Overflow trying to calculate system's frame size")
    }
}

/// Macro for easily creating a [`System`].
///
/// # Example
/// ```
/// use audio_time::system;
///
/// let _ = system!(44_100, Mono, i16);
/// let _ = system!(8_000, Stereo, f64);
/// ```
#[macro_export]
macro_rules! system {
    ($sample_rate:literal, $channel_layout:ident, $sample:ty) => {
        ::audio_time::System {
            sample_rate: ::audio_time::sample_rate!($sample_rate),
            channel_layout: ::audio_time::ChannelLayout::$channel_layout,
            sample_type: ::audio_time::SampleType::new::<$sample>(),
        }
    };
}

/// Audio CD encoding system.
///
/// <https://en.wikipedia.org/wiki/Compact_Disc_Digital_Audio>:
/// ```text
/// 2 channels of LPCM audio, each signed 16-bit values sampled at 44100 Hz
/// ```
pub const AUDIO_CD: System = system!(44_100, Stereo, i16);
