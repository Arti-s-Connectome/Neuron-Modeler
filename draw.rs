use std::f64::consts::TAU;
use eframe::emath::{Pos2, Rect, remap};
use eframe::epaint::Color32;
use egui_plot::{Line, LineStyle, PlotPoints};



// -------------------------------------------------------------------------------------------------

/// Contains a list of shape types.
#[derive(Clone, Debug)]
pub enum ShapeType {
    CIRCLE,
    SQUARE,
    TRIANGLE
}

/// Contains data for drawing a shape.
pub struct Shape {
    center: Pos2,
    radius: f64,
    color: Color32,
    shp_type: ShapeType,
    bounds: Rect
}

// Shape functions
impl Shape {
    /// Creates a circle shape.
    pub fn circle(center: Pos2, radius: f64, color: Color32) -> Self {
        Self {
            center,
            radius,
            color,
            shp_type: ShapeType::CIRCLE,
            bounds: Rect::from_min_max(
                Pos2::new(center.x - radius as f32, center.y - radius as f32),
                Pos2::new(center.x + radius as f32, center.y + radius as f32)
            )
        }
    }

    /// Creates a square shape.
    pub fn square(center: Pos2, radius: f64, color: Color32) -> Self {
        Self {
            center,
            radius,
            color,
            shp_type: ShapeType::SQUARE,
            bounds: Rect::from_min_max(
                Pos2::new(center.x - radius as f32, center.y - radius as f32),
                Pos2::new(center.x + radius as f32, center.y + radius as f32)
            )
        }
    }

    /// Creates a triangle shape.
    pub fn triangle(center: Pos2, radius: f64, color: Color32) -> Self {
        Self {
            center,
            radius,
            color,
            shp_type: ShapeType::TRIANGLE,
            bounds: Rect::from_min_max(
                Pos2::new(center.x - radius as f32, center.y - radius as f32),
                Pos2::new(center.x + radius as f32, center.y + radius as f32)
            )
        }
    }

    /// Draws the shape.
    pub fn draw(&self, hovered: bool) -> Line {
        return match self.shp_type {
            ShapeType::CIRCLE => {
                Line::new(Self::draw_circle(self.center, self.radius))
                    .color(self.color)
                    .style(LineStyle::Solid)
                    .highlight(hovered)
            }
            ShapeType::SQUARE => {
                Line::new(Self::draw_square(self.center, self.radius))
                    .color(self.color)
                    .style(LineStyle::Solid)
                    .highlight(hovered)
            }
            ShapeType::TRIANGLE => {
                Line::new(Self::draw_triangle(self.center, self.radius))
                    .color(self.color)
                    .style(LineStyle::Solid)
                    .highlight(hovered)
            }
        }
    }

    /// Draws the points for a circle
    fn draw_circle(center: Pos2, radius: f64) -> PlotPoints {
        let n = 360;

        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = radius;
                [
                    r * t.cos() + center.x as f64,
                    r * t.sin() + center.y as f64,
                ]
            })
            .collect();

        circle_points
    }

    /// Draws the points for a square.
    fn draw_square(center: Pos2, radius: f64) -> PlotPoints {
        let n = 360;

        let square_points: PlotPoints = (0..=n)
            .map(|i| {
                let r = radius;
                return if i / 90 == 0 {
                    [
                        center.x as f64 - r,
                        center.y as f64 + (((i % 90) as f64 / 89.0) * r * 2.0) - r,
                    ]
                }
                else if i / 90 == 1 {
                    [
                        center.x as f64 + (((i % 90) as f64 / 89.0) * r * 2.0) - r,
                        center.y as f64 + r,
                    ]
                }
                else if i / 90 == 2 {
                    [
                        center.x as f64 + r,
                        center.y as f64 + (((89 - (i % 90)) as f64 / 89.0) * r * 2.0) - r,
                    ]
                }
                else {
                    [
                        center.x as f64 + (((89 - (i % 90)) as f64 / 89.0) * r * 2.0) - r,
                        center.y as f64 - r,
                    ]
                }
            })
            .collect();

        square_points
    }

    /// Draws the points for a triangle.
    fn draw_triangle(center: Pos2, radius: f64) -> PlotPoints {
        let n = 360;

        let triangle_points: PlotPoints = (0..=n)
            .map(|i| {
                let r = radius;
                return if i / 120 == 0 {
                    [
                        center.x as f64 - r + (((i % 120) as f64 / 119.0) * r),
                        center.y as f64 - r + (((i % 120) as f64 / 119.0) * r * 2.0),
                    ]
                }
                else if i / 120 == 1 {
                    [
                        center.x as f64 + (((i % 120) as f64 / 119.0) * r),
                        center.y as f64 + r - (((i % 120) as f64 / 119.0) * r * 2.0),
                    ]
                }
                else {
                    [
                        center.x as f64 + (((119 - (i % 120)) as f64 / 119.0) * r * 2.0) - r,
                        center.y as f64 - r,
                    ]
                }
            })
            .collect();

        triangle_points
    }
}