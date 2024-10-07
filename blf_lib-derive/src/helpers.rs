use syn::{Attribute, DeriveInput};

pub trait DeriveInputHelpers {
    fn get_attribute(&self, name: &str) -> &Attribute;
}

impl DeriveInputHelpers for DeriveInput {
    fn get_attribute(&self, name: &str) -> &Attribute {
        self.attrs.iter().filter(
            |a| a.path().segments.len() == 1 && a.path().segments[0].ident == name
        ).nth(0).unwrap_or_else(|| panic!("{name} attribute required for deriving!"))
    }
}