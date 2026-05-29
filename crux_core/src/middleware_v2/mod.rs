mod resolver;

use crate::middleware_v2::resolver::EffectResolver;

/// Middleware that can process a typed core effect operation without requiring
/// FFI serialization.
pub trait Middleware {
    type Operation: Send + 'static;
    type Output: Send + Unpin + 'static;

    /// Process the operation and resolve its result via the given resolver.
    fn try_process_effect(
        &self,
        operation: Self::Operation,
        resolver: EffectResolver<Self::Output>,
    );
}

/// Default middleware which does not handle any effects.
#[derive(Default)]
pub struct PhantomMiddleware;

impl Middleware for PhantomMiddleware {
    type Operation = ();

    type Output = ();

    fn try_process_effect(&self, _: Self::Operation, mut resolver: EffectResolver<Self::Output>) {
        resolver.resolve(());
    }
}
