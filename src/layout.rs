//! Shared 2D geometry: segment intersection (moved verbatim from
//! `plot.rs` — the legend resolver's muscle, reused not rewritten),
//! plus the diagram node-box model and edge router.
//!
//! Coordinates are TikZ cm on the placed canvas. Node boxes are
//! conservative estimates (the emitter never sees rendered text
//! metrics): half-width 1.7, half-height 0.45 — inside the 4.2/2.4
//! layer grid, so neighbors never collide by construction.

pub(crate) fn pt_in_rect(x: f64, y: f64, r: (f64, f64, f64, f64)) -> bool {
    x >= r.0 && x <= r.2 && y >= r.1 && y <= r.3
}

pub(crate) fn orient(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}

pub(crate) fn on_seg(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> bool {
    cx >= ax.min(bx) && cx <= ax.max(bx) && cy >= ay.min(by) && cy <= ay.max(by)
}

pub(crate) fn segs_cross(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> bool {
    let (o1, o2, o3, o4) = (
        orient(a.0, a.1, b.0, b.1, c.0, c.1),
        orient(a.0, a.1, b.0, b.1, d.0, d.1),
        orient(c.0, c.1, d.0, d.1, a.0, a.1),
        orient(c.0, c.1, d.0, d.1, b.0, b.1),
    );
    ((o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0))
        || (o1 == 0.0 && on_seg(a.0, a.1, b.0, b.1, c.0, c.1))
        || (o2 == 0.0 && on_seg(a.0, a.1, b.0, b.1, d.0, d.1))
        || (o3 == 0.0 && on_seg(c.0, c.1, d.0, d.1, a.0, a.1))
        || (o4 == 0.0 && on_seg(c.0, c.1, d.0, d.1, b.0, b.1))
}

pub(crate) fn seg_hits_rect(x1: f64, y1: f64, x2: f64, y2: f64, r: (f64, f64, f64, f64)) -> bool {
    pt_in_rect(x1, y1, r)
        || pt_in_rect(x2, y2, r)
        || segs_cross((x1, y1), (x2, y2), (r.0, r.1), (r.2, r.1))
        || segs_cross((x1, y1), (x2, y2), (r.2, r.1), (r.2, r.3))
        || segs_cross((x1, y1), (x2, y2), (r.2, r.3), (r.0, r.3))
        || segs_cross((x1, y1), (x2, y2), (r.0, r.3), (r.0, r.1))
}

pub(crate) fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.2 && a.2 > b.0 && a.1 < b.3 && a.3 > b.1
}

/// Conservative node box around a placed center.
pub(crate) fn node_box(x: f64, y: f64) -> (f64, f64, f64, f64) {
    (x - 1.7, y - 0.45, x + 1.7, y + 0.45)
}

/// Route one edge around non-endpoint node boxes.
///
/// Returns `None` when the straight segment is already clear — the
/// caller then emits the exact historical `\draw` line, so clean
/// graphs are provably untouched (golden parity). Otherwise a
/// midpoint pushed perpendicular to the segment until both halves
/// clear all boxes (bounded: 8 pushes of 0.6cm). Exhaustion is
/// `Err` naming the edge — never a line through a node, never a
/// loop.
pub(crate) fn route_edge(
    from: &str,
    to: &str,
    a: (f64, f64),
    b: (f64, f64),
    boxes: &[(String, (f64, f64, f64, f64))],
) -> Result<Option<(f64, f64)>, String> {
    let blocked: Vec<(f64, f64, f64, f64)> = boxes
        .iter()
        .filter(|(id, _)| id != from && id != to)
        .map(|(_, r)| *r)
        .collect();
    if !blocked.iter().any(|r| seg_hits_rect(a.0, a.1, b.0, b.1, *r)) {
        return Ok(None);
    }
    // Perpendicular unit, side away from the first hit box's center
    // (deterministic: no search, no tie-break needed).
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt().max(1e-9);
    let (px, py) = (-dy / len, dx / len);
    let hit = blocked
        .iter()
        .find(|r| seg_hits_rect(a.0, a.1, b.0, b.1, **r))
        .unwrap();
    let (cx, cy) = ((hit.0 + hit.2) / 2.0, (hit.1 + hit.3) / 2.0);
    let (mx, my) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let side = if (mx + px - cx) * px + (my + py - cy) * py >= 0.0 { 1.0 } else { -1.0 };
    for k in 1..=8 {
        let t = 0.6 * k as f64 * side;
        let m = (mx + px * t, my + py * t);
        let clear = !blocked.iter().any(|r| {
            seg_hits_rect(a.0, a.1, m.0, m.1, *r) || seg_hits_rect(m.0, m.1, b.0, b.1, *r)
        });
        if clear {
            return Ok(Some(m));
        }
    }
    Err(format!("edge {from}->{to} crosses a node box; reroute exhausted"))
}

/// Composite/subgraph box around placed member points: pads and the
/// inside-top label position. Layout-first rule — inputs are placed
/// coordinates, outputs never move them.
pub(crate) fn cluster_box(pts: &[(f64, f64)]) -> Option<((f64, f64, f64, f64), (f64, f64))> {
    if pts.is_empty() {
        return None;
    }
    let (x0, x1) = (
        pts.iter().map(|p| p.0).fold(f64::INFINITY, f64::min) - 1.2,
        pts.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max) + 1.2,
    );
    // Screen coords: top is max y. Half-row headroom for the
    // inside-top label (the state-box rule).
    let (yt, yb) = (
        pts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max) + 1.1,
        pts.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) - 0.7,
    );
    Some(((x0, yt, x1, yb), (x0 + 0.1, yt - 0.1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moved_helpers_still_green() {
        // The move is mechanical: same cases the legend resolver
        // relied on, re-asserted at the new home.
        assert!(segs_cross((0.0, 0.0), (2.0, 2.0), (0.0, 2.0), (2.0, 0.0)));
        assert!(!segs_cross((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)));
        assert!(seg_hits_rect(0.0, 0.0, 2.0, 0.0, (0.9, -0.5, 1.1, 0.5)));
        assert!(!seg_hits_rect(0.0, 0.0, 2.0, 0.0, (0.9, 0.6, 1.1, 1.0)));
        assert!(rects_overlap((0.0, 0.0, 1.0, 1.0), (0.5, 0.5, 2.0, 2.0)));
    }

    #[test]
    fn clean_edge_is_none() {
        let b = vec![("c".to_string(), node_box(9.0, 9.0))];
        assert_eq!(route_edge("a", "b", (0.0, 0.0), (2.0, 0.0), &b).unwrap(), None);
    }

    #[test]
    fn push_goes_away_from_the_box() {
        // Vertical edge through a centered box: the midpoint must
        // leave to one side (|x| > 0) and both halves must clear.
        let b = vec![("c".to_string(), node_box(0.0, -2.4))];
        let m = route_edge("a", "d", (0.0, 0.0), (0.0, -4.8), &b).unwrap().unwrap();
        assert!(m.0.abs() > 0.5, "pushed aside, got {m:?}");
        assert!(!seg_hits_rect(0.0, 0.0, m.0, m.1, b[0].1));
        assert!(!seg_hits_rect(m.0, m.1, 0.0, -4.8, b[0].1));
    }

    #[test]
    fn iteration_bound_errors() {
        // A wall wider than the 8-push reach (4.8cm): no sideways
        // midpoint can clear it — error, not a loop, not a line
        // through a node.
        let wall: Vec<(String, (f64, f64, f64, f64))> = (-24..=24)
            .map(|k| (format!("w{k}"), (k as f64 * 0.5 - 0.4, -3.0, k as f64 * 0.5 + 0.4, -1.0)))
            .collect();
        let e = route_edge("a", "b", (0.0, 0.0), (0.0, -4.8), &wall).unwrap_err();
        assert!(e.contains("a->b"), "names the edge, got: {e}");
    }
}
