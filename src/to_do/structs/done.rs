use super::base::Base;
use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;

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

#[cfg(test)]
mod done_test {
    use super::Done;

    #[test]
    fn new() {
        let expected_status: &str = "done";
        let title: &str = "excel date";
        let expected_title: &str = "excel date";
        let done: Done = Done::new(title);

        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}