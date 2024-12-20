
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);

    let radius = win.w().min(win.h()) * 0.4;
    let center = pt2(0.0, 0.0);
    let num_points = 9;
    let radians1 = 180.0 * (std::f32::consts::PI / 180.0);

    #[derive(Clone)]
    #[derive(Copy)]
    struct MyPoint {
        p: Vec2,
        label: usize,
    }

    let mut points:[MyPoint;9]  = [ MyPoint { p: Vec2::new(0.0, 0.0), label: 0 }; 9 ];
    draw.ellipse()
            .stroke(BLUE)
            .xy(center)
            .no_fill()
            .stroke_weight(3.0)
            .radius(radius);

    for i in 0..num_points {
        let mut mv:MyPoint =  MyPoint { p: Vec2::new(0.0, 0.0), label: 0 };
        let angle = (2.0 * PI * (i as f32 / num_points as f32) - PI / 2.0)- radians1;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let point = center + vec2(x, y);
        mv.p.x = point.x;
        mv.p.y = point.y;
        let vv = num_points -i; 
        mv.label = vv;
        points[i]= mv;

    }
    static mut MUTABLE_STATIC: i32 = 0;

    unsafe {
        if MUTABLE_STATIC == 0 {
            MUTABLE_STATIC += 1;
            println!("==============================\n");
            for i in 0..num_points {
                let mv :MyPoint = points[i];
                println!("{},({},{}),{}",i,mv.p.x,mv.p.y,mv.label);
            }
            println!("==============================\n");
        }
    }

    let thickness:f32 = 4.0;
    // 9,6
    draw.line()
            .start(points[0].p)
            .end(points[3].p)
            .color(RED)
            .stroke_weight(thickness);
    // 9,3
    draw.line()
            .start(points[0].p)
            .end(points[6].p)
            .color(RED)
            .stroke_weight(thickness);
    // 1,2
    draw.line()
            .start(points[8].p)
            .end(points[7].p)
            .color(BLUE)
            .stroke_weight(thickness);
    // 2,4
    draw.line()
            .start(points[7].p)
            .end(points[5].p)
            .color(BLUE)
            .stroke_weight(thickness);
    // 4,8
    draw.line()
            .start(points[5].p)
            .end(points[1].p)
            .color(BLUE)
            .stroke_weight(thickness);
    // 8,7
    draw.line()
            .start(points[1].p)
            .end(points[2].p)
            .color(BLUE)
            .stroke_weight(thickness);
    // 7,5
    draw.line()
            .start(points[2].p)
            .end(points[4].p)
            .color(BLUE)
            .stroke_weight(thickness);
    // 5,1
    draw.line()
            .start(points[4].p)
            .end(points[8].p)
            .color(BLUE)
            .stroke_weight(thickness);
    for i in 0..num_points {
        draw.text(&points[i].label.to_string())
            .color(RED)
            .xy(points[i].p)
            .font_size(48);
        
    }
    draw.to_frame(app, &frame).unwrap();
}