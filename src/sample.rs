use std::{fmt, intrinsics::type_id, marker::ConstParamTy, mem::size_of};

use nonzero_const_param::NonZeroU8;

/// A type used to encode a single sample, created from a type that implements
/// [audio_core::Sample].
///
/// This struct "erases" the type it was created with, to allow it to be used as
/// a const generic. On creation, it encodes the size of the sample type, which
/// can be retrieved later using [`byte_depth`](SampleType::byte_depth).
///
/// The struct also encodes the type's unique [`type_id`], so that two
/// `SampleType`s created from different types with the same size are not
/// equal, e.g.:
/// ```
/// # use audio_time::SampleType;
/// #
/// assert_eq!(SampleType::new::<i16>(), SampleType::new::<i16>());
/// assert_ne!(SampleType::new::<i16>(), SampleType::new::<u16>());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ConstParamTy)]
pub struct SampleType {
    byte_depth: NonZeroU8,
    _type: u128,
}

impl fmt::Debug for SampleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SampleType")
            .field("byte_depth", &self.byte_depth())
            .finish()
    }
}

impl SampleType {
    #[inline]
    pub const fn new<Sample: audio_core::Sample + 'static>() -> Self {
        Self {
            byte_depth: NonZeroU8::new(size_of::<Sample>() as u8).unwrap(),
            _type: type_id::<Sample>(),
        }
    }

    /// The [number of bytes](size_of) used to represent this sample type.
    pub const fn byte_depth(&self) -> std::num::NonZeroU8 {
        self.byte_depth.into_std()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_depth() {
        assert_eq!(1, SampleType::new::<i8>().byte_depth().get());
        assert_eq!(2, SampleType::new::<i16>().byte_depth().get());
        assert_eq!(4, SampleType::new::<u32>().byte_depth().get());
        assert_eq!(8, SampleType::new::<f64>().byte_depth().get());
    }
}
