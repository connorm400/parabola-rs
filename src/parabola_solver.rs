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

    let calculate = move |_| {
        todo!()
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
    }
}
