//! random stuff

// stuffing this into a macro 
#[macro_export]
macro_rules! validate_float_input {
    ($setter: ident, $input_element: ident) => {
        $setter($input_element()
            .expect("<input> should exist")
            .value()
            .parse::<f64>()
            .expect("how did that happen"));
    };
}

#[derive(PartialEq, Eq, Debug)]
pub enum MathError {
    Imaginary,
    Undefined
}
impl std::error::Error for MathError {}
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MathError::Imaginary => "an imaginary number",
            MathError::Undefined => "undefined"
        })
    }
}