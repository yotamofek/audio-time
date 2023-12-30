#[macro_export]
macro_rules! impl_fmt {
    ($name:ident) => {
        impl<const SYS: System> ::std::fmt::Display for $name<SYS> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.get(), f)
            }
        }

        impl<const SYS: System> ::std::fmt::Debug for $name<SYS> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self.get(), f)
            }
        }
    };
}
