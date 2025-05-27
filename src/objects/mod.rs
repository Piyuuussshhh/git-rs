pub mod blob;
pub use blob::Blob;

pub mod commit;
pub use commit::Commit;

pub trait GitObject {
    /// For `serialize`, some objects need no extra data (`()`),
    /// others need a `&str` path.  We use a GAT (Generic Associated Type)
    /// so each impl picks its own `Arg<'a>`.
    type SerializerArg<'a>;
    /// Similarly for `deserialize`.
    type DeserializerArg<'b>;

    /// Turn `self` into the raw bytes you’d hash in Git.
    /// The method‐level `<'a>` lets `Arg<'a>` borrow for exactly `'a`.
    fn serialize<'a>(&self, arg: Self::SerializerArg<'a>) -> String;

    /// Reconstruct `Self` from the on‐disk/content‐string form.
    fn deserialize<'b>(&mut self, content: Self::DeserializerArg<'b>)
    where
        Self: Sized;
}