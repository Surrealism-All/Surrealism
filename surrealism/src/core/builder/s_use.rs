use std::fmt::{Display, Formatter};
use super::BaseWrapperImpl;

pub trait UseWrapperImpl<'a>: BaseWrapperImpl {
    fn ns(&mut self, ns: &'a str) -> &mut Self;
    fn db(&mut self, db: &'a str) -> &mut Self;
}

#[derive(Debug, Clone)]
pub struct UseWrapper<'a> {
    ns: &'a str,
    db: &'a str,
}

impl<'a> UseWrapper<'a> {
    pub fn get_ns(&self) -> &'a str {
        self.ns
    }
    pub fn get_db(&self) -> &'a str {
        self.db
    }
}

impl<'a> Default for UseWrapper<'a> {
    fn default() -> Self {
        UseWrapper {
            ns: "test",
            db: "",
        }
    }
}

impl<'a> BaseWrapperImpl for UseWrapper<'a> {
    fn new() -> Self {
        UseWrapper::default()
    }

    fn deref_mut(&mut self) -> Self {
        self.clone()
    }

    fn build(&mut self) -> String {
        format!("{};", self.build_as_child())
    }

    fn build_as_child(&mut self) -> String {
        self.to_string()
    }
}

impl<'a> Display for UseWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("USE");
        let ns = self.get_ns();
        let _ = res.push_str(" NS ");
        let _ = res.push_str(ns);
        let db = self.get_db();
        if !db.is_empty() {
            let _ = res.push_str(" DB ");
            let _ = res.push_str(db);
        }
        write!(f, "{}", res)
    }
}

impl<'a> UseWrapperImpl<'a> for UseWrapper<'a> {
    fn ns(&mut self, ns: &'a str) -> &mut Self {
        self.ns = ns;
        self
    }

    fn db(&mut self, db: &'a str) -> &mut Self {
        self.db = db;
        self
    }
}
