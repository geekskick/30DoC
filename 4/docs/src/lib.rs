#![doc(html_logo_url = "https://www.freefavicon.com/freefavicons/plants/flower-115-152-288275.png")]

pub use bar::Bar;

/// This is re-exported
pub mod bar{
    /// Bar docs
    pub struct Bar;
}

#[doc(inline)]
pub use inlineBar::Bara;

/// This is now not in the 'Re-exports' section of the documentation
pub mod inlineBar{
    /// Bara inline
    pub struct Bara;
}

pub use privateBar::Barp;
/// This module is not documented because it's private
mod privateBar{
    /// Barp private
    pub struct Barp;
}
/// foo is a function
pub fn foo(){}

#[doc(hidden)]
/// This doesn't generate documentation because it's hidden
pub fn hidden_foo(){}

/// Adds 1 to a number
/// # Arguments
/// * `a` - the number to add one to
/// # Returns
/// * `a` + 1
/// # Examples
/// ```rust
/// use docs::add;
/// // this fails when i run cargo test 
/// assert_eq!(add(4), 3);
/// // this passes when i run cargo test
/// assert_eq!(add(4),5);
/// ```
pub fn add(a:i32) -> i32 {
    a + 1
}   
