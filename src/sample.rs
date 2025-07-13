use std::{fmt, marker::ConstParamTy, mem::size_of};

mod sealed {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
    pub struct TypeId(u8);

    /// # Safety
    /// Implementors must make sure to use a unique type ID per implementation.
    pub unsafe trait Sample: audio_core::Sample {
        const TYPE_ID: TypeId;
    }

    macro_rules! impl_sample {
        ($ty:ty, $id:literal) => {
            unsafe impl Sample for $ty {
                const TYPE_ID: TypeId = TypeId($id);
            }
        };
    }

    impl_sample!(u8, 0);
    impl_sample!(u16, 1);
    impl_sample!(u32, 2);
    impl_sample!(u64, 3);
    impl_sample!(u128, 4);
    impl_sample!(i8, 5);
    impl_sample!(i16, 6);
    impl_sample!(i32, 7);
    impl_sample!(i64, 8);
    impl_sample!(i128, 9);
    impl_sample!(usize, 10);
    impl_sample!(isize, 11);
    impl_sample!(f32, 12);
    impl_sample!(f64, 13);
}

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
#[derive(Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
pub struct SampleType {
    byte_depth: NonZeroU8,
    _type: sealed::TypeId,
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
    pub const fn new<Sample: sealed::Sample + 'static>() -> Self {
        Self {
            byte_depth: NonZeroU8::new(size_of::<Sample>() as u8).unwrap(),
            _type: Sample::TYPE_ID,
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
