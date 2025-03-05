mod command;
mod worker;

pub use command::*;
pub use worker::*;

#[cfg(test)]
mod worker_mock;
#[cfg(test)]
pub use worker_mock::tests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Distribution {
    pub id: String,
    pub release: String,
}
