//! random stuff

// leptos has a lot of boilerplate when it comes to components,
// so I'm just stuffing stuff into components and metaprogramming. 
#[macro_export]
macro_rules! controlled_input {
    ($getter: ident, $setter: ident) => {
        view! { 
            <input type="text"
                on:input=move |ev| $setter(event_target_value(&ev))
                prop:value=$getter
                class="thin"
            />
        }
       
    }
}

// stuffing this into a macro 
// used with node refs and setters
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

#[macro_export]
macro_rules! uncontrolled_number_input {
    ($getter: ident, $noderef: ident) => {
        view! {
            <input type="number"
             step=0.0001
             required=true
             value=$getter
             node_ref=$noderef
            />
        }
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