use crate::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub enum Shape {
    Circle {
        radius: f32,
    },
    /// A polygonal shape. NOTE: Points on the exterior should be defined in CLOCKWISE order
    Polygon {
        points: Vec<Vec2>,
    },
}
impl Shape {
    /// Given my placement and a point, figure out the signed distance and the diff need to get to a point on MY border
    /// that is closest to this point. Also return the signed distance from this point to the provided point.
    /// NOTE: The returned point is in GLOBAL, UNROTATED SPACE, relative to MY POS
    pub fn closest_point(&self, placement: (Vec2, f32), rhs: Vec2) -> (f32, Vec2) {
        let (my_pos, my_rot) = placement;
        match self {
            Self::Circle { radius: my_radius } => {
                let diff = rhs - my_pos;
                let signed_dist = diff.length() - *my_radius;
                let norm = diff.normalize_or_zero();
                (signed_dist, my_pos + norm * *my_radius)
            }
            Self::Polygon { points: my_points } => {
                let mut signed_dist = f32::MAX;
                let mut closest_point = Vec2::ZERO;
                for unplaced_line in my_points.to_lines() {
                    let placed_line = [
                        my_pos + unplaced_line[0].my_rotate(my_rot),
                        my_pos + unplaced_line[1].my_rotate(my_rot),
                    ];
                    let (test_signed_dist, test_cp) = signed_distance_to_segment(rhs, placed_line);
                    if test_signed_dist.abs() < signed_dist.abs() {
                        signed_dist = test_signed_dist;
                        closest_point = test_cp;
                    }
                }
                (signed_dist, closest_point)
            }
        }
    }

    /// Given my placement and another shape/placement combo, figure out how to push this shape
    /// out of the other. Returns None if they do not overlap. Otherwise, returns two things:
    /// 1. A diff which represents how much to move my placement by to get out of the shape
    /// 2. The exact collision point
    pub fn bounce_off(
        &self,
        placement: (Vec2, f32),
        rhs: (&Self, Vec2, f32),
    ) -> Option<(Vec2, Vec2)> {
        let (my_pos, _my_rot) = placement;
        let (rhs_bounds, rhs_pos, rhs_rot) = rhs;
        match self {
            Self::Circle { radius: my_radius } => {
                let (signed_dist, cp) = rhs_bounds.closest_point((rhs_pos, rhs_rot), my_pos);
                if signed_dist > *my_radius {
                    return None;
                }
                let dir = (my_pos - cp).normalize_or_zero();
                Some((dir * (*my_radius - signed_dist), cp))
            }
            Self::Polygon { points: _my_points } => {
                unimplemented!("Determining the push point for polygons is not yet supported");
            }
        }
    }
}
impl Shape {
    pub fn to_anim_points(&self) -> Vec<Vec2> {
        match self {
            Self::Circle { radius } => regular_polygon(radius.ceil() as u32, 0.0, *radius),
            Self::Polygon { points } => points.clone(),
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Bounds {
    shape: Shape,
    // TODO IF NEEDED: Add additional info (like a bounding circle, max/min x/y) to speed up collision detection
}
impl Bounds {
    pub fn from_shape(shape: Shape) -> Self {
        Self { shape }
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct Inactive;
