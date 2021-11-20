use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let input_status: String = String::from("done");
        let base: Base = Base::new(input_title, &input_status);
        return Done { super_struct: base };
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}

// cargo test to_do::structs::done
#[cfg(test)]
mod done_test {
    use super::Done;

    #[test]
    fn new() {
        // given
        let expected_status = String::from("done");
        let title = String::from("excel date");
        let expected_title = String::from("excel date");

        // when
        let done = Done::new(&title);

        // then
        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}
