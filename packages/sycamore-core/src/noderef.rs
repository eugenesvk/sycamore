//! References to nodes in views.

use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use sycamore_reactive::*;

use crate::generic_node::GenericNode;

/// A reference to a [`GenericNode`].
/// This allows programmatically accessing the node and call imperative methods on it.
///
/// # Example
/// ```
/// use sycamore::prelude::*;
///
/// #[component]
/// fn Component<G: Html>(cx: Scope) -> View<G> {
///     let my_div = create_node_ref(cx);
///     view! { cx,
///         div(ref=my_div)
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NodeRef<G: GenericNode>(Rc<RefCell<Option<G>>>);

impl<G: GenericNode + Any> NodeRef<G> {
    /// Creates an empty [`NodeRef`].
    ///
    /// Generally, it is preferable to use [`create_node_ref`]
    /// instead.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }

    /// Gets the T stored inside the [`NodeRef`].
    ///
    /// # Panics
    /// Panics if the [`NodeRef`] is not set yet or is the wrong type.
    ///
    /// For a non panicking version, see [`NodeRef::try_get`].
    #[track_caller]
    pub fn get<T: GenericNode>(&self) -> T {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the T stored inside the [`NodeRef`] or `None` if it is not yet set or
    /// the wrong type.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    pub fn try_get<T: GenericNode>(&self) -> Option<T> {
        let obj = self.0.borrow();
        (obj.as_ref()? as &dyn Any).downcast_ref().cloned()
    }

    /// Gets the raw [`GenericNode`] stored inside the [`NodeRef`].
    ///
    /// # Panics
    /// Panics if the [`NodeRef`] is not set yet.
    ///
    /// For a non panicking version, see [`NodeRef::try_get_raw`].
    #[track_caller]
    pub fn get_raw(&self) -> G {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the raw [`GenericNode`] stored inside the [`NodeRef`] or `None` if it is
    /// not yet set.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    pub fn try_get_raw(&self) -> Option<G> {
        self.0.borrow().clone()
    }

    /// Sets the [`NodeRef`] with the specified [`GenericNode`].
    ///
    /// This method should be rarely used. Instead, use the `ref=` syntax in the `view!` macro to
    /// set the node.
    pub fn set(&self, node: G) {
        *self.0.borrow_mut() = Some(node);
    }
}

impl<G: GenericNode> Default for NodeRef<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: GenericNode> fmt::Debug for NodeRef<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeRef").field(&self.0.borrow()).finish()
    }
}

/* Hook implementation */

/// Create a new [`NodeRef`] on the current [`Scope`].
pub fn create_node_ref<G: GenericNode>(cx: Scope<'_>) -> &NodeRef<G> {
    create_ref(cx, NodeRef::new())
}
