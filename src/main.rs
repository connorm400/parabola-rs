use leptos::*;
use parabola_rs::QuadraticFormulaSolver;

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