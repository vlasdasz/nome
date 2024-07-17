use std::fmt::Display;

use crate::{Alert, Spinner};

pub trait AlertErr<T> {
    fn alert_err(self) -> Option<T>;
}

impl<T, E: Display> AlertErr<T> for Result<T, E> {
    fn alert_err(self) -> Option<T> {
        self.inspect_err(|err| {
            Spinner::instant_stop();
            Alert::show(err);
        })
        .ok()
    }
}

impl AlertErr<()> for () {
    fn alert_err(self) -> Option<()> {
        None
    }
}
