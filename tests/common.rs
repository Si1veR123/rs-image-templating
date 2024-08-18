// Miri takes too long to open fonts
#[cfg(not(miri))]
pub mod text;
pub mod filters;