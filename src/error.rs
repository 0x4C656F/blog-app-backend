use std::fmt::Debug;

pub trait ToFieldError<T> {
    fn to_field_error(&self, text: &str) -> Result<T, juniper::FieldError>;
}

impl<T, E> ToFieldError<T> for Result<T, E> where T: Clone, E: Debug {
    fn to_field_error(&self, text: &str) -> Result<T, juniper::FieldError> {
        match self {
            Ok(value) => Ok(value.clone()),
            Err(e) => {
                println!("ToFieldError found an error: {:?}", e);
                Err(juniper::FieldError::new(text, juniper::Value::Null))},
        }
    }
}
