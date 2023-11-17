use leptos::*;
use parabola_rs::controlled_input;

#[component]
pub fn ParabolaSolver() -> impl IntoView {
    // we will use a controlled input for this 
    let (vertex_x, set_vertex_x) = create_signal(0.0);
    let (vertex_y, set_vertex_y) = create_signal(0.0);

    let (point1_x, set_point1_x) = create_signal(0.0);
    let (point1_y, set_point1_y) = create_signal(0.0);
    view! {
        <table> 
            <tr>
                <th>"Vertex"</th>
                <th>"Point on Parabola"</th>
            </tr>
            <tr>
                <th>
                    <label>"("{controlled_input!(vertex_x, set_vertex_x)}", "{controlled_input!(vertex_y, set_vertex_y)}")"</label>
                </th>
                <th>
                    <label>"("{controlled_input!(point1_x, set_point1_x)}", "{controlled_input!(point1_y, set_point1_y)}")"</label>
                </th>
            </tr>
        </table>
        <p>"vertex: ("{vertex_x}", "{vertex_y}")"</p>
        <p>"other point: ("{point1_x}", "{point1_y}")"</p>
    }
}
