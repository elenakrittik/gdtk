use crate::{diagnostic::Diagnostic, protocol::Visualizer};

pub struct Report<'a, V: Visualizer> {
    pub diagnostic: Diagnostic<'a>,
    pub visualizer: V,
}

impl<'a, V: Visualizer> Report<'a, V> {
    pub fn new(diagnostic: Diagnostic<'a>, visualizer: V) -> Self {
        Self { diagnostic, visualizer }
    }

    pub fn submit(self, buf: &mut impl std::fmt::Write) -> Result<(), V::Error> {
        self.visualizer.visualize(self.diagnostic, buf)
    }
}
