pub mod hosting;

mod serving {
    fn take_order() {
        // use with a relative path
        // use super::hosting::add_to_waitlist;

        // use with an absolute path
        use crate::front_of_house::hosting::add_to_waitlist;

        // calling the function brought into scope with the use keyword
        add_to_waitlist();
    }

    fn serve_order() {}

    fn take_payment() {}
}
