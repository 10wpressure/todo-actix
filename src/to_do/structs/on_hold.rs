use crate::to_do::structs::base::Base;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;

pub struct OnHold {
    pub super_struct: Base,
}

impl OnHold {
    pub fn new(input_title: &str) -> Self {
        let base: Base = Base::new(input_title, "on_hold");
        Self { super_struct: base }
    }
}
impl Get for OnHold {}
impl Edit for OnHold {}
