use crate::to_do::structs::base::Base;
use crate::to_do::structs::traits::create::Create;
use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base: Base = Base::new(input_title, "pending");
        Self { super_struct: base }
    }
}
impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}