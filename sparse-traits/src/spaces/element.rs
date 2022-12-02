use super::LinearSpace;

/// Elements of linear spaces.
pub trait Element {
    /// Item type of the vector.
    type Space: LinearSpace;
    type View<'a>
    where
        Self: 'a;
    type ViewMut<'a>
    where
        Self: 'a;

    /// Return the underlying space.
    fn space(&self) -> &Self::Space {
        std::unimplemented!();
    }

    fn view<'a>(&'a self) -> Self::View<'a>;

    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a>;
}

// The view type associated with elements of linear spaces.
pub type ElementView<'a, Space> = <<Space as LinearSpace>::E as Element>::View<'a>;

// The mutable view type associated with elements of linear spaces.
pub type ElementViewMut<'a, Space> = <<Space as LinearSpace>::E as Element>::ViewMut<'a>;
