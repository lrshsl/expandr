use std::{fmt::Debug, marker::PhantomData};

use crate::IntoOwned;

/// How to represent the source
pub trait SourceType: Clone + Debug + IntoOwned {
    type Str: Clone
        + Debug
        + Eq
        + std::hash::Hash
        + ToString
        + AsRef<str>
        + std::borrow::Borrow<str>;
}

/// Owned version of SourceType
///
/// Used to remember the `ProgramContext` of dependencies longer than their source strings are in
/// memory.
#[derive(Clone, Debug)]
pub struct Owned;

impl SourceType for Owned {
    type Str = String;
}

impl IntoOwned for Owned {
    type Owned = Owned;

    fn into_owned(self) -> Self::Owned {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Borrowed<'s>(
    // Tell rustc that 's is needed
    PhantomData<&'s ()>,
);

impl<'s> SourceType for Borrowed<'s> {
    type Str = &'s str;
}

impl IntoOwned for Borrowed<'_> {
    type Owned = Owned;

    fn into_owned(self) -> Self::Owned {
        Owned
    }
}
