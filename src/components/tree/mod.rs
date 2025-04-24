mod family_member;
mod family_section;
mod add_relative_modal;
mod tree_styles;
mod tree;

// Define the Person struct here so it can be used by all modules
#[derive(Clone, PartialEq)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub birth_year: Option<i32>,
    pub profile_image: Option<String>,
    pub relationship: String,
}

// Export the components that need to be used outside this module
pub use family_member::FamilyMember;
pub use family_section::FamilySection;
pub use add_relative_modal::AddRelativeModal;
pub use tree::Tree;
