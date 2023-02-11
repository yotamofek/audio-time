use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelLayout {
    Mono,
    Stereo,
}

impl ChannelLayout {
    pub const fn channels(&self) -> NonZeroU8 {
        NonZeroU8::new(match self {
            Self::Mono => 1,
            Self::Stereo => 2,
        })
        .unwrap()
    }
}
