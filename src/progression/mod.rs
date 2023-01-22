pub trait Progressable {
    /// Returns a vector containing all available progressions from self
    fn get_progressions(&self) -> Vec<Self>
    where
        Self: Sized;
}
