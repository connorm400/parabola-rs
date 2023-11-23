use leptos::*;
use parabola_rs::controlled_input;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ReactivePoint {
    x: ReadSignal<f64>,
    x_setter: WriteSignal<f64>,
    y: ReadSignal<f64>,
    y_setter: WriteSignal<f64>,
    enabled: ReadSignal<bool>,
    enabled_setter: WriteSignal<bool>
}

impl ReactivePoint {
    fn new() -> Self {
        let (x, x_setter) = create_signal(0.0);
        let (y, y_setter) = create_signal(0.0);
        let (enabled, enabled_setter) = create_signal(false);
        ReactivePoint { 
            x, x_setter,
            y, y_setter,
            enabled, enabled_setter 
        }
    }
}

#[component]
fn Point(point:  Rc<ReactivePoint>) -> impl IntoView {
    // I have to destructure this stuff to fix my macro
    let x = point.x;
    let x_setter = point.x_setter;
    let y = point.y;
    let y_setter = point.y_setter;
    view! {<label>"("{controlled_input!(x, x_setter)}", "{controlled_input!(y, y_setter)}")"</label>}
}

#[component]
fn Checkbox(point: Rc<ReactivePoint>) -> impl IntoView {
    let setter = point.enabled_setter;
    let getter = point.enabled;
    view! {
        <input type="checkbox"
        on:input=move |ev| {
            setter(event_target_checked(&ev));
        }
        prop:value=getter
        />
    }
}
    

#[component]
pub fn ParabolaSolver() -> impl IntoView {
    let vertex = Rc::new(ReactivePoint::new());
    let point1 = Rc::new(ReactivePoint::new());
    let point2 = Rc::new(ReactivePoint::new());

    let (answer, set_answer) = create_signal(String::new());

    // I need to make a new reference to move into the calculate closure.
    let (calc_vertex, calc_point1, calc_point2) = (vertex.clone(), point1.clone(), point2.clone());
    let calculate = move |_| {
        logging::log!("vertex: {calc_vertex:?}, point 1: {calc_point1:?}, point 2: {calc_point2:?}");
        let vertex = (calc_vertex.x.get(), calc_vertex.y.get());
        let point1 = (calc_point1.x.get(), calc_point1.y.get());
        let point2 = (calc_point2.x.get(), calc_point2.y.get());
        
        match (calc_vertex.enabled.get(), calc_point1.enabled.get(), calc_point2.enabled.get()) {
            (true, true, _ ) => set_answer(format!("{}", find_parabola_with_vertex(vertex, point1))),
            (true, _,  true) => set_answer(format!("{}", find_parabola_with_vertex(vertex, point2))),
            _ => set_answer("not implemented".to_string())
        }
    };

    view! {
        <table> 
            <tr>
                <th>"Vertex"</th>
                <th>"Point on Parabola"</th>
                <th>"Other Point (Optional)"</th>
            </tr>
            <tr>
                <th><label>"using this point?"</label><Checkbox point=vertex.clone()/></th>
                <th><label>"using this point?"</label><Checkbox point=point1.clone()/></th>
                <th><label>"using this point?"</label><Checkbox point=point2.clone()/></th>
            </tr>
            <tr>
                <th><Point point=vertex.clone()/></th>
                <th><Point point=point1.clone()/></th>
                <th><Point point=point2.clone()/></th>
            </tr>
        </table>
    
        <input type="submit" 
            on:click=calculate
            class="calculate"
            value="kalculate!"
        />

        <p>"➡️ Quadratic formula in standard form: "{answer}</p>

    }
}

#[derive(Debug, PartialEq)]
struct Parabola {
    vertex: (f64, f64),
    /// coefficient is the a part in standard form y = a(x - h)^2 + k
    coefficient : f64,
}

impl std::fmt::Display for Parabola {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (h, k) = self.vertex;
        write!(f, "y = {:.1}(x - {h})^2 + {k}", self.coefficient)
    }
}
/// This will calculate the parabola given one points and a vertex.
/// mostly finds the coefficient and puts the rest into the Parabola struct.
fn find_parabola_with_vertex((x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> Parabola {
    Parabola { 
        vertex: (x0, y0), 
        coefficient: (y1 - y0)/(x1 - x0).powf(2.0) 
    }
}

#[allow(unused)]
fn find_parabola_with_two_point((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> Result<Parabola, ()> {
    todo!()
}