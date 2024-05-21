trait Archive {
    type Archived;
}

trait Deserialize<T> {
    fn deserialize(&self);
}

trait ArchivePointee {
    type ArchivedMetadata;
}

struct RelPtr<T: ArchivePointee + ?Sized> {
    metadata: T::ArchivedMetadata,
}

struct ArchivedVec<T>(RelPtr<[T]>);

#[inline]
fn as_slice<T>(v: &RelPtr<[T]>) {
    let _a: <[T] as ArchivePointee>::ArchivedMetadata = v.metadata;
}

impl<T: Archive> Deserialize<T> for ArchivedVec<T::Archived>
where
    [T::Archived]: ArchivePointee,
{
    fn deserialize(&self) {
        as_slice(&self.0);
    }
}

impl<T> ArchivePointee for [T] {
    type ArchivedMetadata = u32;
}
