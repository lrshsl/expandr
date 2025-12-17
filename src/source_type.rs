use std::{fmt::Debug, marker::PhantomData};

/// How to represent the source
pub trait SourceType: Clone + Debug {
    type Str: Clone + Debug + PartialEq + ToString + AsRef<str>;
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

#[derive(Clone, Debug)]
pub struct Borrowed<'s>(
    // Tell rustc that 's is needed
    PhantomData<&'s ()>,
);

impl<'s> SourceType for Borrowed<'s> {
    type Str = &'s str;
}
