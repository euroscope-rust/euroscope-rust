//! `CSectorElement` — a reference to an element of the loaded sector file
//! (VOR, NDB, airport, runway, SID/STAR, etc.).

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{Context, Position, SectorElementType, api::handle::scoped, utils::cstr};

/// A reference to a sector-file element.
///
/// A thin handle over EuroScope's `CSectorElement`, valid only for the
/// duration of the callback / query block that produced it (`'cb`).
#[derive(Clone, Copy)]
pub struct SectorElement<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl SectorElement<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CSectorElement*`. Escape hatch for `euroscope-sys` calls not
    /// yet wrapped here.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether this handle references a valid sector element.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_sectorelement_is_valid(self.raw) }
    }

    /// The element type code.
    pub fn element_type(&self) -> i32 {
        // SAFETY: `raw` is a live handle for this callback.
        unsafe { euroscope_sys::es_sectorelement_element_type(self.raw) }
    }

    /// The element name. Borrows EuroScope-owned memory; valid for this
    /// callback only.
    pub fn name(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_sectorelement_name(self.raw)) }
    }

    /// The position at `index` (starting from zero). Returns `None` if the
    /// index is invalid for this element (not all elements have coordinates).
    pub fn position(&self, index: i32) -> Option<Position> {
        let mut lat = 0.0_f64;
        let mut lon = 0.0_f64;
        // SAFETY: `raw` is a live handle; the out-params are valid stack slots.
        let ok = unsafe {
            euroscope_sys::es_sectorelement_position(self.raw, index, &raw mut lat, &raw mut lon)
        };
        ok.then(|| Position::new(lat, lon))
    }

    /// The name of the switchable component at `index` (starting from zero).
    /// Empty if the index is invalid. Borrows EuroScope-owned memory; valid
    /// for this callback only.
    pub fn component_name(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe {
            cstr(euroscope_sys::es_sectorelement_component_name(
                self.raw, index,
            ))
        }
    }

    /// The frequency of VOR/NDB/AIRPORT elements, or `0.0` for all others.
    pub fn frequency(&self) -> f64 {
        // SAFETY: `raw` is a live handle for this callback.
        unsafe { euroscope_sys::es_sectorelement_frequency(self.raw) }
    }

    /// The name of the runway at `index` (0 or 1). Empty if the index is
    /// invalid. Borrows EuroScope-owned memory; valid for this callback only.
    pub fn runway_name(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_sectorelement_runway_name(self.raw, index)) }
    }

    /// The heading of the runway at `index` (0 or 1), or `-1` if the index is
    /// invalid.
    pub fn runway_heading(&self, index: i32) -> i32 {
        // SAFETY: `raw` is a live handle for this callback.
        unsafe { euroscope_sys::es_sectorelement_runway_heading(self.raw, index) }
    }

    /// The name of the airport this element belongs to. Empty if the type is
    /// invalid. Borrows EuroScope-owned memory; valid for this callback only.
    pub fn airport_name(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_sectorelement_airport_name(self.raw)) }
    }

    /// Whether the element (airport or runway) is active for `departure`
    /// (`true`) or arrival (`false`). `index` (0 or 1) selects the runway and
    /// is ignored for non-runway elements.
    pub fn is_element_active(&self, departure: bool, index: i32) -> bool {
        // SAFETY: `raw` is a live handle for this callback.
        unsafe { euroscope_sys::es_sectorelement_is_element_active(self.raw, departure, index) }
    }
}

scoped!(SectorElementHandle, SectorElement, es_sectorelement_free);

/// Iterator over sector-file elements of a given [`SectorElementType`], yielding
/// callback-scoped handles. Its selectors carry the element-type filter, so it
/// does not use the generic iterator macro.
pub struct SectorElements<'cb> {
    plugin: euroscope_sys::PluginPtr,
    element_type: i32,
    next: Option<EsHandle>,
    _marker: PhantomData<&'cb ()>,
}

impl SectorElements<'_> {
    pub(crate) fn new(plugin: euroscope_sys::PluginPtr, element_type: i32) -> Self {
        // SAFETY: `plugin` is EuroScope's live CPlugIn*.
        let first =
            unsafe { euroscope_sys::es_plugin_sectorelement_select_first(plugin, element_type) };
        Self {
            plugin,
            element_type,
            next: (!first.is_null()).then_some(first),
            _marker: PhantomData,
        }
    }
}

#[expect(
    clippy::missing_trait_methods,
    reason = "We don't need the extra methods for this type"
)]
impl<'cb> Iterator for SectorElements<'cb> {
    type Item = SectorElementHandle<'cb>;

    fn next(&mut self) -> Option<SectorElementHandle<'cb>> {
        let cur = self.next.take()?;
        // SAFETY: `plugin`/`cur` live; shim returns a heap copy or null.
        let following = unsafe {
            euroscope_sys::es_plugin_sectorelement_select_next(self.plugin, cur, self.element_type)
        };
        self.next = (!following.is_null()).then_some(following);
        Some(SectorElementHandle {
            inner: SectorElement::from_raw(cur),
        })
    }
}

#[expect(clippy::missing_trait_methods, reason = "We don't need pin_drop")]
impl Drop for SectorElements<'_> {
    fn drop(&mut self) {
        if let Some(p) = self.next.take() {
            // SAFETY: `p` is a heap handle the shim allocated.
            unsafe { euroscope_sys::es_sectorelement_free(p) }
        }
    }
}

/// Sector-file element iteration.
impl Context {
    /// Iterate over sector-file elements of the given type. Pass
    /// [`SectorElementType::All`] for every element.
    pub fn sector_file_elements(&self, element_type: SectorElementType) -> SectorElements<'_> {
        // SAFETY: `Context` holds a live CPlugIn* for the plugin's lifetime.
        SectorElements::new(unsafe { self.as_ptr() }, element_type.to_raw())
    }
}
