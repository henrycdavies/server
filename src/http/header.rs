use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Header<'buf> {
    name: &'buf str,
    value: &'buf str,
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}:{}", self.name, self.value)
    }
}

impl<'buf> From<&'buf str> for Header<'buf> {
    fn from(s: &'buf str) -> Self {
        let (n, v) = s.split_once(":").unwrap();
        Header { name: n.trim(), value: v.trim_start()}
    }
}