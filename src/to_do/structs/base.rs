use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Self {
        Self {
            title: input_title.to_string(),
            status: input_status.to_string(),
        }
    }
}

#[cfg(test)]
mod base_tests {
    use super::Base;

    #[test]
    fn new() {
        let title: &str = "test title";
        let expected_title: &str = "test title";
        let status: &str = "test status";
        let expected_status: &str = "test status";

        let new_base_struct: Base = Base::new(title, status);
        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}