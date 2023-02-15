use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title, "done");
        Done { super_struct: base }
    }
}

#[cfg(test)]
mod done_test {
    use super::*;

    #[test]
    fn new() {
        let expected_status: String = String::from("done");
        let title: String = String::from("excel date");
        let expected_title: String = String::from("excel date");
        let done: Done = Done::new(&title);
        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}
