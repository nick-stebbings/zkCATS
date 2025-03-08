// use derive_more::From;
// use lib_core::model;

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;
// #[derive(Debug, From)]
// pub enum Error {
//     // -- Modules
//     // #[from]
//     // Model(model::Error),
// }
