use std::{f32::consts::PI, iter::once};

use lyon::{
    math::point,
    path::Path,
    tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
};

use crate::{
    flat::{Point, Rect},
    num::into_f32::ToF32,
};

pub struct PointsPath {}

impl PointsPath {
    pub fn circle_with(center: impl Into<Point>, radius: impl ToF32, precision: u16) -> Vec<Point> {
        let radius = radius.to_f32();
        let center = center.into();
        let angle_step = PI * 2.0 / f32::from(precision);
        (0..precision)
            .map(|i| point_on_circle(radius, angle_step * f32::from(i), center))
            .collect()
    }

    pub fn circle_triangles_with(center: impl Into<Point>, radius: impl ToF32, precision: u16) -> Vec<Point> {
        let radius = radius.to_f32();
        let center = center.into();
        let circle = Self::circle_with(center, radius, precision);

        pairs(&circle).into_iter().flat_map(|(a, b)| [a, b, center]).collect()
    }

    pub fn rounded_rect(rect: impl Into<Rect>, radius: impl ToF32, precision: u16) -> Vec<Point> {
        let mut path = vec![];
        let rect = rect.into();
        let radius = radius.to_f32();

        let a = (rect.x() + radius, rect.y() + radius);
        let b = (rect.max_x() - radius, rect.y() + radius);
        let c = (rect.max_x() - radius, rect.max_y() - radius);
        let d = (rect.x() + radius, rect.max_y() - radius);

        let angle_step = PI * 0.5 / f32::from(precision);

        for i in 0..precision {
            path.push(point_on_circle(radius, -3.0 + angle_step * f32::from(i), a));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, -1.5 + angle_step * f32::from(i), b));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, angle_step * f32::from(i), c));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, 1.5 + angle_step * f32::from(i), d));
        }

        path
    }

    pub fn tessellate(points: Vec<Point>) -> (Vec<Point>, Vec<u16>) {
        assert!(!points.is_empty(), "Empty tessellate");

        let mut builder = Path::builder();

        points.into_iter().enumerate().for_each(|(i, p)| {
            if i == 0 {
                builder.begin(point(p.x, p.y));
            } else {
                builder.line_to(point(p.x, p.y));
            }
        });

        builder.end(true);

        let path = builder.build();

        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        {
            tessellator
                .tessellate_path(
                    &path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Point {
                        x: vertex.position().x,
                        y: vertex.position().y,
                    }),
                )
                .unwrap();
        }

        (geometry.vertices, geometry.indices)
    }
}

pub fn point_on_circle(radius: f32, angle: f32, center: impl Into<Point>) -> Point {
    let center = center.into();
    (radius * angle.cos() + center.x, radius * angle.sin() + center.y).into()
}

fn pairs<T: Copy>(data: &[T]) -> Vec<(T, T)> {
    let first = *data.first().unwrap();
    let last = *data.last().unwrap();
    data.windows(2)
        .chain(once([first, last].as_ref()))
        .map(|data| (data[1], data[0]))
        .collect()
}

#[test]
fn test_pairs() {
    assert_eq!(pairs(&[1, 2, 3]), vec![(2, 1), (3, 2), (3, 1)]);
}

#[test]
fn test_tessellate() {
    let concave_points = vec![
        Point { x: -16.89, y: 4.16 },
        Point { x: 8.59, y: 11.09 },
        Point { x: 11.99, y: -0.36 },
        Point { x: -9.97, y: -9.34 },
        Point { x: -3.92, y: -0.85 },
    ];

    PointsPath::tessellate(concave_points);
}
