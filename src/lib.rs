pub mod ds;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn rc() {
        #[derive(Default)]
        struct User {
            username: String,
        }
        let u = Rc::new(RefCell::new(User {
            username: String::from("user1"),
        }));
        assert_eq!(u.borrow().username, String::from("user1"));
        u.borrow_mut().username = String::from("updated user name");
        assert_eq!(u.borrow().username, String::from("updated user name"));
    }
}
