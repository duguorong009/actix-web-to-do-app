use super::{
    base::Base,
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        Pending { super_struct: base }
    }
}

impl Create for Pending {}

impl Delete for Pending {}

impl Edit for Pending {}

impl Get for Pending {}
