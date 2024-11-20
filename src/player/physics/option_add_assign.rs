use std::ops::Add;

pub(super) trait OptionAddAssignExtension<T> {
    fn add_assign(&mut self, value: T) -> &mut Self;
}

impl<T: Add<Output = T> + Default + Clone> OptionAddAssignExtension<T> for Option<T> {
    fn add_assign(&mut self, value: T) -> &mut Self {
        *self = Some(self.clone().unwrap_or_default() + value); self
    }
}