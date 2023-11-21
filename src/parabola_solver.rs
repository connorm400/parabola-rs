use leptos::*;
use parabola_rs::controlled_input;

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
fn Point(
    /*
    x: ReadSignal<f64>, x_setter: WriteSignal<f64>,
    y: ReadSignal<f64>, y_setter: WriteSignal<f64>
    */
    point: ReactivePoint
) -> impl IntoView {
    let x = point.x;
    let x_setter = point.x_setter;
    let y = point.y;
    let y_setter = point.y_setter;
    view! {<label>"("{controlled_input!(x, x_setter)}", "{controlled_input!(y, y_setter)}")"</label>}
}

#[component]
pub fn ParabolaSolver() -> impl IntoView {
    let vertex = ReactivePoint::new();
    let point1 = ReactivePoint::new();
    let point2 = ReactivePoint::new();

    let calculate = move |_| {
        ()
    };

    view! {
        <table> 
            <tr>
                <th>"Vertex"</th>
                <th>"Point on Parabola"</th>
                <th>"Other Point (Optional)"</th>
            </tr>
            <tr> 
            </tr>
            <tr>
                <th>
                    <Point point=vertex/>
                </th>
                <th>
                    <Point point=point1/>
                </th>
                <th>
                   <Point point=point2/>
                </th>
            </tr>
        </table>
    
        /*
        <p>"vertex: ("{vertex_x}", "{vertex_y}")"</p>
        <p>"other point: ("{point1_x}", "{point1_y}")"</p>
        <p>"another reactive point: ("{point2_x}","{point2_y}")"</p>
        */
        <input type="submit" 
            on:click=calculate
            class="calculate"
            value="kalculate!"
        />
    }
}
