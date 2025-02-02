use triomphe::{Arc, UniqueArc};

use crate::freeze::Freezer;

pub enum BoxedArcCow<T>
where
    T: 'static,
{
    Arc(Arc<T>),
    Boxed(UniqueArc<T>),
}

impl_traits!(BoxedArcCow, Boxed);

impl<T> BoxedArcCow<T> {
    /// This is deep freeze, but doesn't work if `self <- Freezed <- NonFreezed`
    /// exists.
    #[inline]
    pub fn freeze(&mut self)
    where
        Self: VisitMutWith<Freezer>,
    {
        self.visit_mut_with(&mut Freezer)
    }
}

impl<T> From<T> for BoxedArcCow<T> {
    fn from(data: T) -> Self {
        BoxedArcCow::Boxed(UniqueArc::new(data))
    }
}

impl<T> BoxedArcCow<T>
where
    T: Clone,
{
    #[inline]
    pub fn into_inner(self) -> T {
        match self {
            Self::Arc(v) => match Arc::try_unwrap(v) {
                Ok(v) => v,
                Err(v) => (*v).clone(),
            },
            Self::Boxed(v) => UniqueArc::into_inner(v),
        }
    }
}
