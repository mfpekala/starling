use crate::prelude::*;

/// Given a point and a line segment, get the point on the line segment
/// that is closest to the provided point
pub fn closest_point_on_segment(pos: Vec2, line: [Vec2; 2]) -> Vec2 {
    let l2 = (line[1].x - line[0].x).powi(2) + (line[1].y - line[0].y).powi(2);
    let t = ((pos.x - line[0].x) * (line[1].x - line[0].x)
        + (pos.y - line[0].y) * (line[1].y - line[0].y))
        / l2;
    let t = t.clamp(0.0, 1.0);
    Vec2 {
        x: line[0].x + t * (line[1].x - line[0].x),
        y: line[0].y + t * (line[1].y - line[0].y),
    }
}

/// Calculates the signed distance from a point to a line segment. Also returns the closest point
/// Returns a POSITIVE number if the pos is "OUTSIDE" the line according to our clockwise convention
/// Returns a NEGATIVE number if the pos is "INSIDE" the line according to our clockwise convention.
pub fn signed_distance_to_segment(pos: Vec2, line: [Vec2; 2]) -> (f32, Vec2) {
    let cp = closest_point_on_segment(pos, line);
    let line_diff = line[1] - line[0];
    let normal_pointing = Vec2 {
        x: line_diff.y,
        y: -line_diff.x,
    };
    let diff = line[0] - pos;
    let dotprod = diff.dot(normal_pointing);
    (dotprod.signum() * pos.distance(cp), cp)
}

pub trait ToLines {
    fn to_lines(&self) -> Vec<[Vec2; 2]>;
}

impl ToLines for Vec<Vec2> {
    fn to_lines(&self) -> Vec<[Vec2; 2]> {
        let mut result = vec![[Vec2::ZERO, Vec2::ZERO]; self.len()];
        for ix in 0..self.len() {
            result[ix] = [self[ix], self[(ix + 1).rem_euclid(self.len())]];
        }
        result
    }
}

/// I am small-brain, this is the rotation api I want
pub trait MyRotations {
    fn my_rotate(self, angle: f32) -> Self;
}

impl MyRotations for Vec2 {
    fn my_rotate(self, angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Vec2::new(self.x * c - self.y * s, self.x * s + self.y * c)
    }
}

/// Bevy API is not quite what I want
pub trait MyTranNAngleGetter {
    fn tran_n_angle(&self) -> (Vec2, f32);
}

impl MyTranNAngleGetter for Transform {
    fn tran_n_angle(&self) -> (Vec2, f32) {
        (
            self.translation.truncate(),
            self.rotation.to_euler(EulerRot::XYZ).2,
        )
    }
}

impl MyTranNAngleGetter for GlobalTransform {
    fn tran_n_angle(&self) -> (Vec2, f32) {
        let (_, quat, tran) = self.to_scale_rotation_translation();
        (tran.truncate(), quat.to_euler(EulerRot::XYZ).2)
    }
}

/// Just trying to make it dead-easy to do anything with rotation, that's all
pub trait MyAngleSetter {
    fn set_angle(&mut self, angle: f32);
}

impl MyAngleSetter for Transform {
    fn set_angle(&mut self, angle: f32) {
        self.rotation = Quat::from_rotation_z(angle)
    }
}

/// Helpful function to generate the points of a rectangle, centered at zero, with our clockwise convention
pub fn simple_rect(width: f32, height: f32) -> Vec<Vec2> {
    vec![
        Vec2::new(-width / 2.0, -height / 2.0),
        Vec2::new(width / 2.0, -height / 2.0),
        Vec2::new(width / 2.0, height / 2.0),
        Vec2::new(-width / 2.0, height / 2.0),
    ]
}

/// Given a list of points, return points that retain the same shape, but produce an outline
pub fn outline_points(points: &[Vec2], width: f32) -> Vec<Vec2> {
    let mut new_points = vec![];
    for (ix, point) in points.iter().enumerate() {
        let point_before = points[if ix == 0 { points.len() - 1 } else { ix - 1 }];
        let point_after = points[if ix == points.len() - 1 { 0 } else { ix + 1 }];
        let slope_before = (*point - point_before).normalize_or_zero();
        let slope_after = (point_after - *point).normalize_or_zero();
        let tang = (slope_before + slope_after).normalize_or_zero();
        let perp = Vec2::new(-tang.y, tang.x);
        new_points.push(*point + perp * width);
    }
    new_points
}

/// Returns the smallest UVec 2 such that an aabb of that size could cover points
pub fn uvec2_bound(points: &[Vec2]) -> UVec2 {
    let mut mins = Vec2::new(f32::MAX, f32::MAX);
    let mut maxs = Vec2::new(f32::MIN, f32::MIN);
    for vec in points {
        mins = mins.min(*vec);
        maxs = maxs.max(*vec);
    }
    UVec2 {
        x: (maxs.x - mins.x).ceil() as u32,
        y: (maxs.y - mins.y).ceil() as u32,
    }
}
