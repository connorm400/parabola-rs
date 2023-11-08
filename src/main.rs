use leptos::*;
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! {
        <h1>"Parabola Solver"</h1>
        <hr/>
        <h2>"Quadratic Formula Solver"</h2>
        <div class="block">   
            <QuadraticFormulaSolver/>
        </div>
    });
}

// stuffing this into a macro 
macro_rules! validate_float_input {
    ($setter: ident, $input_element: ident) => {
        $setter($input_element()
            .expect("<input> should exist")
            .value()
            .parse::<f64>()
            .expect("how did that happen"))
        
    };
}

#[component]
fn QuadraticFormulaSolver() -> impl IntoView {
    // creating a bunch of reactive getters and setters
    let (a, set_a) = create_signal(0.0);
    let input_element_a: NodeRef<html::Input> = create_node_ref();

    let (b, set_b) = create_signal(0.0);
    let input_element_b: NodeRef<html::Input> = create_node_ref();

    let (c, set_c) = create_signal(0.0);
    let input_element_c: NodeRef<html::Input> = create_node_ref();

    let (answer, set_answer) = create_signal("(,)".to_string());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        validate_float_input!(set_a, input_element_a);
        validate_float_input!(set_b, input_element_b);
        validate_float_input!(set_c, input_element_c);

        set_answer(match quadratic_formula(a(), b(), c()) {
            Ok((a, b)) if a == b => format!("{}", a),
            Ok((a, b)) => format!("({},{})", a, b),
            Err(e) => e
        });
    };

    view! {
        
        <form on:submit=on_submit>
            <input type="number"
             step=0.01
             required="true"
             value=a
             node_ref=input_element_a
            />
            <input type="number"
             step=0.01
             required="true"
             value=b
             node_ref=input_element_b
            />
            <input type="number"
             step=0.01
             required="true"
             value=c
             node_ref=input_element_c
            />
            <input type="submit"
              value="calculate!"
            />
        </form>

        //<p>"Answer a:"{a}" b:"{b}" c:"{c}</p>
        <p>"➡️ Quadratic formula answer: " {answer}</p>
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    // putting this into a variable to reuse it
    let mut discriminant = b.powf(2.0) - 4_f64 * a * c;
    // check if the answer would be imagninary
    if discriminant < 0.0 { return Err("imaginary number".into())}
    // square root it
    discriminant = discriminant.sqrt();
    //return answer (rust has implicit returns where the last expression is returned)
    Ok(((-b + discriminant)/(2.0 * a), (-b - discriminant)/(2.0 * a)))
}

#[cfg(test)]
mod tests {
    use crate::quadratic_formula;

    #[test]
    fn test_quadratic_formula() {
        assert_eq!(Ok((3.0, 2.0)), quadratic_formula(1.0, -5.0, 6.0));
        assert_eq!(Ok((2.0, 2.0)), quadratic_formula(1.0, -4.0, 4.0));
        // if the descriminant is negative it will return an error
        assert_eq!(Err(String::from("imaginary number")), quadratic_formula(4.0, 2.0, 6.0));
    }
}