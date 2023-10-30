use comfy::*;

pub fn try_get_lines_intersection_point(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let d1 = Vec2 { x: a2.x - a1.x, y: a2.y - a1.y };
    let d2 = Vec2 { x: b2.x - b1.x, y: b2.y - b1.y };

    let det = d1.x * d2.y - d1.y * d2.x;

    if det.abs() < 1e-8 {
        // The lines are parallel
        return None;
    }

    let dx = a1.x - b1.x;
    let dy = a1.y - b1.y;

    let t1 = (dy * d2.x - dx * d2.y) / det;
    let t2 = (dy * d1.x - dx * d1.y) / det;

    if t1 < 0.0 || t1 > 1.0 || t2 < 0.0 || t2 > 1.0 {
        return None;
    }

    return Some(Vec2 { x: a1.x + t1 * d1.x, y: a1.y + t1 * d1.y });
}

pub fn closest_points_on_segment(point: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    let ab = b - a;
    let ap = point - a;

    let t = ap.dot(ab) / ab.dot(ab);
    
    // clamp to [0, 1] to stay within segment
    let t_clamped = t.max(0.0).min(1.0);
    
    // Calculate the closest point
    a + ab * t_clamped
}

pub fn closest_points_on_segments(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> (Vec2, Vec2) {
    let intersection_point = try_get_lines_intersection_point(a1, a2, b1, b2);

    if let Some(point) = intersection_point {
        return (point, point);
    }

    let mut candidates = Vec::new();

    candidates.push((a1, closest_points_on_segment(a1, b1, b2)));
    candidates.push((a2, closest_points_on_segment(a2, b1, b2)));

    candidates.push((b1, closest_points_on_segment(b1, a1, a2)));
    candidates.push((b2, closest_points_on_segment(b2, a1, a2)));

    let mut best_distance: Option<f32> = None;
    let mut best_points: Option<(Vec2, Vec2)> = None;

    for (p1, p2) in candidates.iter() {
        let distance = (*p1 - *p2).length();

        if best_distance.is_none() || distance < best_distance.unwrap() {
            best_distance = Some(distance);
            best_points = Some((*p1, *p2));
        }
    }

    return best_points.unwrap();
}
