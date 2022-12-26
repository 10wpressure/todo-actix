use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base: Base = Base::new(input_title, "done");
        Self {
            super_struct: base,
        }
    }
}
impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}