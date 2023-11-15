use leptos::*;
use parabola_rs::{QuadraticFormulaSolver, quadratic_formula};

fn main() {
    println!("{:?}", quadratic_formula(0.0, 0.0, 0.0));
    mount_to_body(|| view! {
        <h1>"Parabola Solver"</h1>
        <hr/>
        <h2>"Quadratic Formula Solver"</h2>
        <div class="block">   
            <QuadraticFormulaSolver/>
        </div>
        <br/> <br/>
        <div class="block">
            <h2>"some other thing"</h2>
        </div>
    });
}