use bevy::math::{DVec2, Vec2};

#[derive(Debug)]
pub struct OrcaAgent {
    pub position: DVec2,
    pub velocity: DVec2,
    pub radius: f64,
}

impl OrcaAgent {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        Self {
            position: position.as_dvec2(),
            velocity: velocity.as_dvec2(),
            radius: radius as f64,
        }
    }
}

const ORCA_EPSILON: f64 = 0.00001;

pub fn orca(
    agent: OrcaAgent,
    neighbors: &[OrcaAgent],
    opt_vel: Vec2,
    radius: f32,
    tau: f32,
) -> Vec2 {
    let opt_vel = opt_vel.as_dvec2();
    let radius = radius as f64;
    let tau = tau as f64;
    let mut lines = Vec::new();
    for neighbor in neighbors.iter() {
        lines.push(compute_orca_line(&agent, neighbor, tau));
    }
    let new_vel = match linear_2(&lines, radius, opt_vel, false) {
        Ok(result) => result,
        Err((start_idx, start_point)) => linear_3(&lines, radius, start_idx, start_point),
    };
    new_vel.as_vec2()
}

fn compute_orca_line(agent: &OrcaAgent, other: &OrcaAgent, tau: f64) -> Line {
    let rel_pos = other.position - agent.position;
    let rel_vel = agent.velocity - other.velocity;
    let dist_sq = rel_pos.length_squared();
    let combined_radius = agent.radius + other.radius;
    let combined_radius_sq = combined_radius * combined_radius;

    let w = rel_vel - rel_pos / tau;
    let w_length_sq = w.length_squared();
    let dot = DVec2::dot(w, rel_pos);

    let (u, dir) = if dist_sq < combined_radius_sq
        || (dot < 0.0 && dot * dot > combined_radius_sq * w_length_sq)
    {
        // project on cut-off circle
        let w_length = w_length_sq.sqrt();
        let unit_w = w / w_length;
        let u = (combined_radius / tau - w_length) * unit_w;
        let dir = DVec2::new(unit_w.y, -unit_w.x);
        (u, dir)
    } else {
        // project on legs
        let leg = (dist_sq - combined_radius_sq).sqrt();
        let dir = if DVec2::perp_dot(rel_pos, w) > 0.0 {
            // project on left leg
            let dir_x = rel_pos.x * leg - rel_pos.y * combined_radius;
            let dir_y = rel_pos.x * combined_radius + rel_pos.y * leg;
            DVec2::new(dir_x, dir_y) / dist_sq
        } else {
            // project on right leg
            let dir_x = rel_pos.x * leg + rel_pos.y * combined_radius;
            let dir_y = -rel_pos.x * combined_radius + rel_pos.y * leg;
            -DVec2::new(dir_x, dir_y) / dist_sq
        };
        let u = DVec2::dot(rel_vel, dir) * dir - rel_vel;
        (u, dir)
    };

    Line::new(agent.velocity + 0.5 * u, dir)
}

fn linear_1(
    line: &Line,
    prev_lines: &[Line],
    radius: f64,
    opt_vel: DVec2,
    opt_dir: bool,
) -> Option<DVec2> {
    let dot = DVec2::dot(line.point, line.direction);
    let discriminant = dot * dot + radius * radius - line.point.length_squared();

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let mut left = -dot - sqrt_discriminant;
    let mut right = -dot + sqrt_discriminant;

    for prev_line in prev_lines.iter() {
        let numerator = DVec2::perp_dot(prev_line.direction, line.point - prev_line.point);
        let denominator = DVec2::perp_dot(line.direction, prev_line.direction);

        if denominator.abs() <= ORCA_EPSILON {
            if numerator < 0.0 {
                return None;
            }
            continue;
        }

        let t = numerator / denominator;

        if denominator >= 0.0 {
            right = f64::min(t, right);
        } else {
            left = f64::max(t, left);
        }

        if left > right {
            return None;
        }
    }

    let point = if opt_dir {
        if DVec2::dot(opt_vel, line.direction) < 0.0 {
            line.point + left * line.direction
        } else {
            line.point + right * line.direction
        }
    } else {
        let t = DVec2::dot(line.direction, opt_vel - line.point);
        if t < left {
            line.point + left * line.direction
        } else if t > right {
            line.point + right * line.direction
        } else {
            line.point + t * line.direction
        }
    };

    Some(point)
}

fn linear_2(
    lines: &[Line],
    radius: f64,
    opt_vel: DVec2,
    opt_dir: bool,
) -> Result<DVec2, (usize, DVec2)> {
    let mut result = match opt_dir {
        true => opt_vel * radius,
        false => opt_vel.clamp_length_max(radius),
    };
    for (i, line) in lines.iter().enumerate() {
        if DVec2::perp_dot(line.direction, line.point - result) <= 0.0 {
            continue;
        }
        result = match linear_1(line, &lines[..i], radius, opt_vel, opt_dir) {
            Some(new_result) => new_result,
            None => return Err((i, result)),
        };
    }
    Ok(result)
}

fn linear_3(lines: &[Line], radius: f64, start_idx: usize, start_point: DVec2) -> DVec2 {
    let mut result = start_point;
    let mut distance = 0.0;
    for (i, line) in lines[start_idx..].iter().enumerate() {
        if DVec2::perp_dot(line.direction, line.point - result) <= distance {
            continue;
        }
        let mut new_lines = Vec::new();
        for prev_line in &lines[..start_idx + i] {
            let determinant = DVec2::perp_dot(line.direction, prev_line.direction);
            let point = match determinant.abs() <= ORCA_EPSILON {
                true => match DVec2::dot(line.direction, prev_line.direction) <= 0.0 {
                    true => 0.5 * (line.point + prev_line.point),
                    false => continue,
                },
                false => {
                    line.point
                        + DVec2::perp_dot(prev_line.direction, line.point - prev_line.point)
                            / determinant
                            * line.direction
                }
            };
            let direction = (prev_line.direction - line.direction).normalize();
            new_lines.push(Line::new(point, direction));
        }
        let opt_vel = line.direction.perp();
        if let Ok(new_result) = linear_2(&new_lines, radius, opt_vel, true) {
            result = new_result;
        }
        distance = DVec2::perp_dot(line.direction, line.point - result);
    }
    result
}

struct Line {
    pub point: DVec2,
    pub direction: DVec2,
}

impl Line {
    pub fn new(point: DVec2, direction: DVec2) -> Self {
        Self { point, direction }
    }
}
