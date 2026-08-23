//! Shared machinery for callback-scoped owned handles and their iterators.
//!
//! EuroScope's `Select*`/`GetCorrelated*` methods return handles by value; the
//! shim heap-copies them (`new CX(...)`) so a pointer survives the FFI call, and
//! the Rust wrapper frees that copy on drop. These wrappers are **scoped to the
//! callback** via a `'cb` lifetime tied to the borrow that produced them (the
//! `&Context`, or the parent handle): the SDK only guarantees a handle resolves
//! "inside the block you are querying", so storing one past the callback would
//! let you dereference a stale index. The lifetime makes that a borrow-check
//! error rather than latent undefined behaviour.
//!
//! These macros generate the `XHandle<'cb>` wrapper (frees its copy on drop,
//! derefs to the borrowed `X<'cb>`) and the `SelectFirst`/`SelectNext` iterator.
//! Each entity module (`controller`, `flight_plan`, …) invokes them next to its
//! own borrowed handle type. The one genuinely persistent handle
//! (`register_fp_list`) is hand-written separately, not generated here.
//!
//! The macros use fully-qualified paths throughout, so an invoking module needs
//! no extra imports beyond `use crate::api::handle::{iter, scoped};`.

/// Generate a callback-scoped owned handle: heap-frees on drop, derefs to the
/// borrowed `$handle<'cb>`. `$free` is the matching `euroscope_sys` deallocator.
macro_rules! scoped {
    ($owned:ident, $handle:ident, $free:ident) => {
        #[doc = concat!("A callback-scoped [`", stringify!($handle), "`](crate::",
            stringify!($handle), ") handle obtained from a query or iteration. Frees its heap \
            copy on drop and derefs to the borrowed handle; the `'cb` lifetime keeps it from \
            being stored past the callback.")]
        pub struct $owned<'cb> {
            inner: crate::$handle<'cb>,
        }

        impl<'cb> $owned<'cb> {
            // Not every scoped type is produced via a selector (some only via
            // iterators, which construct the struct directly), so this may be
            // unused for a given instantiation.
            #[allow(dead_code)]
            pub(crate) fn from_owned(
                raw: euroscope_sys::EsHandle,
            ) -> ::core::option::Option<Self> {
                (!raw.is_null()).then(|| Self {
                    inner: crate::$handle::from_raw(raw),
                })
            }
        }

        impl<'cb> ::core::ops::Deref for $owned<'cb> {
            type Target = crate::$handle<'cb>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        #[expect(clippy::missing_trait_methods, reason = "We don't need pin_drop")]
        impl ::core::ops::Drop for $owned<'_> {
            fn drop(&mut self) {
                // SAFETY: `inner` owns a heap handle the shim allocated for us.
                unsafe { euroscope_sys::$free(self.inner.as_ptr()) }
            }
        }
    };
}
pub(crate) use scoped;

/// Generate an iterator over a `$first`/`$next(current)` selector sequence,
/// yielding callback-scoped `$owned<'cb>` handles and freeing any un-yielded
/// tail on drop.
macro_rules! iter {
    ($iter:ident, $owned:ident, $handle:ident, $free:ident, $first:ident, $next:ident) => {
        #[doc = concat!("Iterator over all `", stringify!($handle),
                                                    "`s, yielding callback-scoped owned handles.")]
        pub struct $iter<'cb> {
            plugin: euroscope_sys::PluginPtr,
            next: ::core::option::Option<euroscope_sys::EsHandle>,
            _marker: ::core::marker::PhantomData<&'cb ()>,
        }

        impl<'cb> $iter<'cb> {
            pub(crate) fn new(plugin: euroscope_sys::PluginPtr) -> Self {
                // SAFETY: `plugin` is EuroScope's live CPlugIn*.
                let first = unsafe { euroscope_sys::$first(plugin) };
                Self {
                    plugin,
                    next: (!first.is_null()).then_some(first),
                    _marker: ::core::marker::PhantomData,
                }
            }
        }

        #[expect(
            clippy::missing_trait_methods,
            reason = "We don't need the extra methods for this type"
        )]
        impl<'cb> ::core::iter::Iterator for $iter<'cb> {
            type Item = $owned<'cb>;

            fn next(&mut self) -> ::core::option::Option<$owned<'cb>> {
                let cur = self.next.take()?;
                // Advance before yielding `cur`: SelectNext needs `cur` alive,
                // and `cur` is then handed to the yielded owned handle.
                // SAFETY: `plugin`/`cur` live; shim returns a heap copy or null.
                let following = unsafe { euroscope_sys::$next(self.plugin, cur) };
                self.next = (!following.is_null()).then_some(following);
                ::core::option::Option::Some($owned {
                    inner: crate::$handle::from_raw(cur),
                })
            }
        }

        #[expect(clippy::missing_trait_methods, reason = "We don't need pin_drop")]
        impl ::core::ops::Drop for $iter<'_> {
            fn drop(&mut self) {
                if let ::core::option::Option::Some(p) = self.next.take() {
                    // SAFETY: `p` is a heap handle the shim allocated.
                    unsafe { euroscope_sys::$free(p) }
                }
            }
        }
    };
}
pub(crate) use iter;
