// The TNamed class is the base class for all named ROOT classes
// A TNamed contains the essential elements (name, title)
// to identify a derived object in containers, directories and files.
// Most member functions defined in this base class are in general
// overridden by the derived classes.

#[derive(Default)]
pub struct Named {
    pub(crate) name: String,
    pub(crate) title: String,
}
