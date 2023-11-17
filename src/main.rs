use leptos::*;
use quadratic_formula_solver::QuadraticFormulaSolver;
use parabola_solver::ParabolaSolver;

mod quadratic_formula_solver;
mod parabola_solver;

fn main() {
    mount_to_body(|| view! {
        <h1>"Ronalds Universal Number Kalkulator"</h1>
        <hr/>
        <div class="block">
            <h2>"Parabola Solver"</h2>
            <ParabolaSolver/>
        </div>

        <div class="block">   
            <h2>"Quadratic Equation Solver"</h2>
            <p>
                "The quadratic equation is a handy formula to solve degree 2 trinomials where the answer is 0. " <br/>
                "You would rearange your polynomial to look like this: "<b>"ax"<sup>2</sup>" + bx + c = 0"</b>
            </p>
            <QuadraticFormulaSolver/>
        </div>
        
    });
}