use leptos::*;
use std::rc::Rc;
use parabola_rs::controlled_input;

#[derive(Debug)]
struct ReactivePoint {
    x: (ReadSignal<String>, WriteSignal<String>),
    y: (ReadSignal<String>, WriteSignal<String>),
}

impl ReactivePoint {
    fn new() -> Rc<Self> {
        Rc::new(ReactivePoint {
            x: create_signal("0.0".into()),
            y: create_signal("0.0".into())
        })
    }
}

#[component]
fn Point(point:  Rc<ReactivePoint>) -> impl IntoView {
    // I have to destructure this stuff to fix my macro
    let (x, x_setter) = point.x;
    let (y, y_setter) = point.y;
    view! {<label>"("{controlled_input!(x, x_setter)}", "{controlled_input!(y, y_setter)}")"</label>}
}

#[component]
pub fn ParabolaSolver() -> impl IntoView {
    let (vertex_method, set_vertex_method) = create_signal(false);
    let (other_method, set_other_method) = create_signal(false);
    let (answer, set_answer) = create_signal("...".to_string());
    let vertex = ReactivePoint::new();
    let point = ReactivePoint::new();

    // for the other method
    let point1 = ReactivePoint::new();
    let point2 = ReactivePoint::new();
    let point3 = ReactivePoint::new();

    // making a new reference for the closure
    let vertex_calc = vertex.clone();
    let point0_calc = point.clone();
    let point1_calc = point1.clone();
    let point2_calc = point2.clone();
    let point3_calc = point3.clone();

    let calculate = move |_| {
        // bad code
        let vertex_x = vertex_calc.x.0.get().trim().parse();
        let vertex_y = vertex_calc.y.0.get().trim().parse();

        let point0_x = point0_calc.x.0.get().trim().parse();
        let point0_y = point0_calc.y.0.get().trim().parse();

        let point1_x = point1_calc.x.0.get().trim().parse();
        let point1_y = point1_calc.y.0.get().trim().parse();

        let point2_x = point2_calc.x.0.get().trim().parse();
        let point2_y = point2_calc.y.0.get().trim().parse();
    
        let point3_x = point3_calc.x.0.get().trim().parse();
        let point3_y = point3_calc.y.0.get().trim().parse();

        set_answer(if vertex_method.get() {
            // terrible code basically check if all inputs are numbers without consuming anything
            match ((vertex_x, vertex_y), (point0_x, point0_y)) {
                    ((Ok(x1), Ok(y1)), (Ok(x2), Ok(y2))) => {
                        format!("{}", match find_parabola_with_vertex((x1, y1), (x2, y2)) {
                            Ok(a) => a.standard_form(),
                            Err(_) => "Thats impossible (divide by zero)".into(),
                        })
                    },
                    _ => "All inputs must be numbers".into(),
            }
        } else if other_method.get() {
            match ((point1_x, point1_y), (point2_x, point2_y), (point3_x, point3_y)) {
                ((Ok(x1), Ok(y1)), (Ok(x2), Ok(y2)), (Ok(x3), Ok(y3))) => {
                    if let Ok(parabola) = find_parabola_with_three_points((Point::new(x1, y1), Point::new(x2, y2), Point::new(x3, y3))) {
                        parabola.standard_form()
                    } else {
                        "Thats impossible".into()
                    }
                },
                _ => "All inputs must be numbers".into(),
            }
        } else {
            "...".into() 
        })
    };

    view! {
        
        <input type="radio" name="method" 
            //this is a little jank but it works :)
            on:input =move |ev| {
                let vertex_radio = event_target_checked(&ev);
                set_vertex_method(vertex_radio);
                set_other_method(!vertex_radio);
            }
            prop:value=vertex_method/>
        <label>"I have the vertex"</label>
        <label>" | "</label>
        
        <input type="radio" name="method" 
        on:input=move |ev| {
            let vertex_radio = event_target_checked(&ev);
            set_other_method(vertex_radio);
            set_vertex_method(!vertex_radio);
        }
            prop:value=other_method/>
        <label>"I have three points on the parabola"</label>

        <br/>
        <Show 
            when=vertex_method
            fallback=move || view! {/* empty */}>
            <hr style="width: 20%; margin-left: 1%;"/>
            <label>"vertex: "</label><br/><Point point=vertex.clone()/>
            <br/>
            <label>"other point on the parabola: "</label><br/><Point point=point.clone()/>
            <br/>
        </Show>

        <Show 
            when=other_method
            fallback=move || view! {}>
            <hr style="width: 20%; margin-left: 1%;"/>
            <label>"point 1: "</label><Point point=point1.clone()/>
            <br/>
            <label>"Point 2: "</label><Point point=point2.clone()/>
            <br/>
            <label>"Point 3: "</label><Point point=point3.clone()/>
            <br/>
        </Show>

        <input type="submit" 
            on:click=calculate
            class="calculate"
            value="kalculate!"
        />

        <p>"➡️ Quadratic formula in standard form: "{answer}</p>
    }
}

#[derive(Debug, PartialEq)]
pub struct Parabola {
    vertex: (f64, f64),
    /// coefficient is the a part in standard form y = a(x - h)^2 + k
    coefficient : f64,
}

impl std::fmt::Display for Parabola {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.standard_form())
    }
}
impl Parabola {
    pub fn standard_form(&self) -> String {
        let (h, k) = self.vertex;
        let h = if h < 0.0 { format!(" + {}", h.abs())} else if h == 0.0 { String::new() } else { format!(" - {h}") };
        let k = if k < 0.0 { format!(" - {}", k.abs())} else if k == 0.0 { String::new() } else { format!(" + {k}") };
        let a = if self.coefficient == 1.0 { String::new() } else { format!("{:.2}", self.coefficient) };
        
        format!("y = {a}(x{h})^2{k}")
    }
}

/// This will calculate the parabola given one points and a vertex.
/// mostly finds the coefficient and puts the rest into the Parabola struct.
fn find_parabola_with_vertex((x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> Result<Parabola, Impossible> {
    if x1 -x0 == 0.0 {
        return Err(Impossible);
    }
    Ok(Parabola { 
        vertex: (x0, y0), 
        coefficient: (y1 - y0)/(x1 - x0).powf(2.0) 
    })
}

pub struct Point {
    x: f64, 
    y:f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Impossible;
pub fn find_parabola_with_three_points((p1, p2, p3): (Point, Point, Point)) -> Result<Parabola, Impossible> {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

    if x1 == x2 || x2 == x3 || x1 == x3 {
        return Err(Impossible);
    }

    let a = ((y1 - y3) * (x2-x3) - (y2 - y3) * (x1 - x3)) / ((x1 - x3) * (x2 - x3) * (x1 - x2));
    let b = (y1 - y3 - a * (x1.powf(2.0) - x3.powf(2.0))) / (x1 - x3);
    let c = y1 - a*x1.powf(2.0) - b*x1;

    let x = -b/(2.0*a);
    let y = a*x.powf(2.0) + b*x + c;
    
    Ok(Parabola {
        vertex: (x, y),
        coefficient: a
    })
}
