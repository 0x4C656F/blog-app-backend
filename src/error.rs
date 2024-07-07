pub trait ToFieldError<T> {
    fn to_field_error(&self, text: &str) -> Result<T, juniper::FieldError>;
}

impl<T, E> ToFieldError<T> for Result<T, E>
where
    T: Clone,
    E: Clone,
{
    fn to_field_error(&self, text: &str) -> Result<T, juniper::FieldError> {
        self.clone()
            .map_err(|_| juniper::FieldError::new(text, juniper::Value::Null))
    }
}
