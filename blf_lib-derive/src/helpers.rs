use syn::{Attribute, DeriveInput};

pub trait DeriveInputHelpers {
    fn get_required_attribute(&self, name: &str) -> &Attribute;
    fn get_attribute(&self, name: &str) -> Option<&Attribute>;
}

impl DeriveInputHelpers for DeriveInput {
    fn get_required_attribute(&self, name: &str) -> &Attribute {
        self.get_attribute(name).unwrap_or_else(|| panic!("{name} attribute required for deriving!"))
    }

    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attrs.iter().filter(
            |a| a.path().segments.len() == 1 && a.path().segments[0].ident == name
        ).nth(0)
    }
}