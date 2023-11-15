use leptos::*;
use web_sys::SubmitEvent;

// stuffing this into a macro 
macro_rules! validate_float_input {
    ($setter: ident, $input_element: ident) => {
        $setter($input_element()
            .expect("<input> should exist")
            .value()
            .parse::<f64>()
            .expect("how did that happen"));
    };
}

#[component]
pub fn QuadraticFormulaSolver() -> impl IntoView {
    // creating a bunch of reactive getters and setters for each input element
    let (a, set_a) = create_signal(0.0);
    let input_element_a: NodeRef<html::Input> = create_node_ref();

    let (b, set_b) = create_signal(0.0);
    let input_element_b: NodeRef<html::Input> = create_node_ref();

    let (c, set_c) = create_signal(0.0);
    let input_element_c: NodeRef<html::Input> = create_node_ref();

    let (answer, set_answer) = create_signal("...".to_string());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        validate_float_input!(set_a, input_element_a);
        validate_float_input!(set_b, input_element_b);
        validate_float_input!(set_c, input_element_c);

        set_answer(match quadratic_formula(a(), b(), c()) {
            Ok((a, b)) if a == b => format!("{a}."),
            Ok((a, b)) => format!("{a} and {b}."),
            Err(e) => format!("{e}."),
        });
    };

    view! {
        
        <form on:submit=on_submit>
            <label>"A: "</label>
            <input type="number"
             step=0.01
             required=true
             value=a
             node_ref=input_element_a
            />
            
            <label>" B: "</label>
            <input type="number"
             step=0.01
             required=true
             value=b
             node_ref=input_element_b
            />
            
            <label>" C: "</label>
            <input type="number"
             step=0.01
             required=true
             value=c
             node_ref=input_element_c
            />
        
            <br/>
            <input type="submit"
              value="calculate!"
            />
        </form>

        <p>"➡️ Quadratic formula answer is " {answer}</p>
    }
}

// dw about this its just some boiler plate for the custom ImaginaryNumber error
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

pub fn quadratic_formula(a: f64, b: f64, c: f64) -> Result<(f64, f64), MathError> {
    for i in [a, b, c].iter() {
        if *i == 0_f64 { return Err(MathError::Undefined)}
    }
    // putting this into a variable to reuse it
    let mut discriminant = b.powf(2.0) - 4_f64 * a * c;
    // check if the answer would be imagninary
    if discriminant < 0.0 { return Err(MathError::Imaginary)}
    // square root it
    discriminant = discriminant.sqrt();
    //return answer (rust has implicit returns where the last expression is returned)
    Ok(((-b + discriminant)/(2.0 * a), (-b - discriminant)/(2.0 * a)))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_quadratic_formula() {
        assert_eq!(Ok((3.0, 2.0)), quadratic_formula(1.0, -5.0, 6.0));
        assert_eq!(Ok((2.0, 2.0)), quadratic_formula(1.0, -4.0, 4.0));
        // if the descriminant is negative it will return an error
        assert_eq!(Err(MathError::Imaginary), quadratic_formula(4.0, 2.0, 6.0));
        assert_eq!(Err(MathError::Undefined), quadratic_formula(0.0, 0.0, 0.0));
    }
}