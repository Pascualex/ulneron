use bevy::prelude::*;

pub type OrcaTransform = (Vec2, f32, Vec2);

pub fn orca(
    trans_a: OrcaTransform,
    vel_pref: Vec2,
    trans_neighbors: Vec<OrcaTransform>,
    tau: f32,
) -> Vec2 {
    let mut half_planes = Vec::new();
    for trans_b in trans_neighbors {
        half_planes.push(half_plane(trans_a, trans_b, tau));
    }
    match closest_point_on_half_planes(&half_planes, vel_pref) {
        Some(some) => some,
        None => Vec2::ZERO,
    }
}

fn half_plane(trans_a: OrcaTransform, trans_b: OrcaTransform, tau: f32) -> HalfPlane {
    let (pos_a, radius_a, vel_a) = trans_a;
    let (pos_b, radius_b, vel_b) = trans_b;

    let vel_diff = vel_a - vel_b;

    let vo_main_pos = pos_b - pos_a;
    let vo_main_radius = radius_a + radius_b;
    let vo_trunc_pos = vo_main_pos / tau;
    let vo_trunc_radius = vo_main_radius / tau;

    let n = (vel_diff - vo_trunc_pos).normalize();
    let closest_ca = vo_trunc_pos + n * vo_trunc_radius;
    let u = closest_ca - vel_diff;

    HalfPlane::new(vel_a + u / 2.0, n)
}

fn closest_point_on_half_planes(half_planes: &[HalfPlane], optimal_point: Vec2) -> Option<Vec2> {
    let mut new_point = optimal_point;
    for (i, half_plane) in half_planes.iter().enumerate() {
        if half_plane.contains(new_point) {
            continue;
        }
        let (left, right) = match half_plane.intersect(&half_planes[..i]) {
            Some(some) => some,
            None => return None,
        };
        let l0 = half_plane.point;
        let dir = Vec2::new(half_plane.normal.y, -half_plane.normal.x);
        let l1 = l0 + dir;
        new_point = closest_point_on_line(l0, l1, left, right, optimal_point)
    }
    Some(new_point)
}

fn closest_point_on_line(l0: Vec2, l1: Vec2, left: f32, right: f32, optimal_point: Vec2) -> Vec2 {
    let c = optimal_point - l0;
    let mut v = l1 - l0;
    let distance = v.length();
    if distance == 0.0 {
        return l0;
    }
    v /= distance;
    let d = (l0 - l1).length();
    let t = c.dot(v) / d;
    l0 + (l1 - l0) * t.clamp(left, right)
}

#[derive(Debug)]
struct HalfPlane {
    point: Vec2,
    normal: Vec2,
}

impl HalfPlane {
    pub fn new(point: Vec2, normal: Vec2) -> Self {
        Self { point, normal }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        (point - self.point).dot(self.normal) > 0.0
    }

    pub fn intersect(&self, half_planes: &[HalfPlane]) -> Option<(f32, f32)> {
        let mut left = -f32::INFINITY;
        let mut right = f32::INFINITY;

        let self_dir = Vec2::new(self.normal.y, -self.normal.x);
        for half_plane in half_planes {
            let dir = Vec2::new(half_plane.normal.y, -half_plane.normal.x);
            let num = (half_plane.point - self.point).perp_dot(dir);
            let den = self_dir.perp_dot(dir);

            if den < f32::EPSILON {
                if num < f32::EPSILON {
                    return None;
                }
                continue;
            }

            let t = num / den;
            if den > 0.0 {
                right = right.min(t);
            } else {
                left = left.max(t);
            }

            if left > right {
                return None;
            }
        }

        Some((left, right))
    }
}
