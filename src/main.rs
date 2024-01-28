use std::path::Path;

use lineararith::Vector;
use optimizable::Optimizable;
use partial_dual::PartialDual;
use svg::{
    node::element::{Circle, Line, Text},
    Document,
};

mod lineararith;
mod optimizable;
mod partial_dual;

struct Example {
    connections: Vec<(usize, usize)>,
}

const POINTS: usize = 10;
const VARNUM: usize = POINTS * 2;

impl Optimizable<VARNUM> for Example {
    fn loss(&self, vars: [f64; VARNUM]) -> PartialDual<VARNUM> {
        let points = (0..POINTS)
            .map(|i| {
                Vector([
                    PartialDual::select(vars, i * 2),
                    PartialDual::select(vars, i * 2 + 1),
                ])
            })
            .collect::<Vec<_>>();

        let mut acc: PartialDual<VARNUM> = Default::default();

        // All points should be as close to the origin as possible
        for point in &points {
            acc = acc + point.length_square();
        }

        // Every point should try to be as far from any other as possible
        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let p1 = points[i];
                let p2 = points[j];
                let dist = (p1 - p2).length_square();
                acc = acc + dist.recip();
            }
        }

        // Every connected point should be as close to each other as possible
        for (i, j) in &self.connections {
            let p1 = points[*i];
            let p2 = points[*j];
            acc = acc + (p1 - p2).length() * 10.0.into();
        }

        acc
    }

    /*fn report(&self, i: usize, vars: &[f64; VARNUM]) {
        if i % 10 == 0 {
            let points: [Vector<f64, 2>; POINTS] = vars
                .chunks_exact(2)
                .map(|a| Vector([a[0], a[1]]))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            render_points(
                &self.connections,
                points,
                format!("frames/frame{:04}.svg", i/10),
            )
        }
    }*/
}

fn render_points<const N: usize, P: AsRef<Path>>(
    connections: &Vec<(usize, usize)>,
    points: [Vector<f64, 2>; N],
    filename: P,
) {
    const WIDTH: f64 = 5.0;
    const HEIGHT: f64 = 5.0;

    let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));

    let axis_x = Line::new()
        .set("x1", 0.0)
        .set("y1", HEIGHT * 0.5)
        .set("x2", WIDTH)
        .set("y2", HEIGHT * 0.5)
        .set("stroke-width", 0.01)
        .set("stroke", "black");
    let axis_y = Line::new()
        .set("x1", WIDTH * 0.5)
        .set("y1", 0)
        .set("x2", WIDTH * 0.5)
        .set("y2", HEIGHT)
        .set("stroke-width", 0.01)
        .set("stroke", "black");

    document = document.add(axis_y).add(axis_x);

    for (i, j) in connections {
        let p1 = points[*i];
        let p2 = points[*j];
        let line = Line::new()
            .set("stroke-width", 0.01)
            .set("stroke", "gray")
            .set("x1", p1.0[0] + WIDTH * 0.5)
            .set("y1", p1.0[1] + HEIGHT * 0.5)
            .set("x2", p2.0[0] + WIDTH * 0.5)
            .set("y2", p2.0[1] + HEIGHT * 0.5);
        document = document.add(line);
    }

    for (i, point) in points.into_iter().enumerate() {
        let x = point.0[0] + WIDTH * 0.5;
        let y = point.0[1] + HEIGHT * 0.5;
        let c = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 0.1)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 0.01);
        let label = Text::new()
            .add(svg::node::Text::new(i.to_string()))
            .set("x", x)
            .set("y", y)
            .set(
                "style",
                "
            font: normal 0.1px sans-serif;
            text-anchor: middle;
            dominant-baseline: central;
            ",
            );
        document = document.add(c).add(label);
    }

    svg::save(filename, &document).unwrap();
}

fn main() {
    let opt = Example {
        connections: vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (3, 4),
            (1, 2),
            (3, 7),
            (0, 6),
            (1, 9),
            (9, 8),
            (5, 9),
            (4, 5),
        ],
    };
    let vars = (0..POINTS)
        .map(|i| {
            let t = (i as f64) / (POINTS as f64) * 6.28318;
            [t.sin() * 1.0, t.cos() * 1.0]
        })
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let result = opt.optimize(vars, 1000, 0.01);
    let loss = opt.loss(result).val;
    println!("{:?}", loss);
    let points: [Vector<f64, 2>; POINTS] = result
        .chunks_exact(2)
        .map(|a| Vector([a[0], a[1]]))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    render_points(&opt.connections, points, "image.svg");
}
