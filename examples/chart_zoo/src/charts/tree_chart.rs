//! Tree/Dendrogram Chart Widget
//!
//! Hierarchical tree layout with smooth bezier curves and GPU-accelerated rendering.
//! Features animated node expansion and gradient links.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawChartLine, DrawPoint};
use super::animation::{ChartAnimator, EasingType, get_color};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;

    pub TreeChartWidget = {{TreeChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone)]
struct TreeNode {
    name: String,
    children: Vec<usize>,
    x: f64,
    y: f64,
    depth: usize,
}

#[derive(Live, LiveHook, Widget)]
pub struct TreeChartWidget {
    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    nodes: Vec<TreeNode>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust(true)]
    horizontal_layout: bool,

    #[rust(true)]
    show_smooth_curves: bool,
}

impl Widget for TreeChartWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.chart_rect = rect;

            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_tree(cx);
        }

        DrawStep::done()
    }
}

impl TreeChartWidget {
    fn initialize_data(&mut self) {
        // Build a sample tree structure
        self.nodes = vec![
            TreeNode { name: "Root".into(), children: vec![1, 2, 3], x: 0.0, y: 0.0, depth: 0 },
            // Level 1
            TreeNode { name: "A".into(), children: vec![4, 5], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "B".into(), children: vec![6, 7, 8], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "C".into(), children: vec![9], x: 0.0, y: 0.0, depth: 1 },
            // Level 2 under A
            TreeNode { name: "A1".into(), children: vec![10, 11], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "A2".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            // Level 2 under B
            TreeNode { name: "B1".into(), children: vec![12], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "B2".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "B3".into(), children: vec![13, 14], x: 0.0, y: 0.0, depth: 2 },
            // Level 2 under C
            TreeNode { name: "C1".into(), children: vec![15], x: 0.0, y: 0.0, depth: 2 },
            // Level 3 leaves
            TreeNode { name: "A1a".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "A1b".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "B1a".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "B3a".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "B3b".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "C1a".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
        ];

        self.compute_layout();
    }

    fn compute_layout(&mut self) {
        let max_depth = self.nodes.iter().map(|n| n.depth).max().unwrap_or(0);

        // First pass: assign leaf positions
        let mut leaf_count = 0usize;
        for node in &mut self.nodes {
            if node.children.is_empty() {
                node.x = leaf_count as f64;
                leaf_count += 1;
            }
        }

        // Second pass: position internal nodes at center of children (bottom-up)
        for depth in (0..max_depth).rev() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].depth == depth && !self.nodes[i].children.is_empty() {
                    let children = self.nodes[i].children.clone();
                    let sum_x: f64 = children.iter().map(|&c| self.nodes[c].x).sum();
                    self.nodes[i].x = sum_x / children.len() as f64;
                }
            }
        }

        // Normalize x positions
        let max_x = self.nodes.iter().map(|n| n.x).fold(0.0f64, |a, b| a.max(b));
        if max_x > 0.0 {
            for node in &mut self.nodes {
                node.x /= max_x;
            }
        }

        // Set y positions based on depth
        if max_depth > 0 {
            for node in &mut self.nodes {
                node.y = node.depth as f64 / max_depth as f64;
            }
        }
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_tree(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;
        let padding = 50.0;

        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        if chart_w <= 0.0 || chart_h <= 0.0 || self.nodes.is_empty() {
            return;
        }

        // Clone nodes for iteration
        let nodes: Vec<_> = self.nodes.clone();
        let max_depth = nodes.iter().map(|n| n.depth).max().unwrap_or(0);

        // Draw links first (behind nodes)
        for node in &nodes {
            let (px, py) = if self.horizontal_layout {
                (chart_x + node.y * chart_w, chart_y + node.x * chart_h)
            } else {
                (chart_x + node.x * chart_w, chart_y + node.y * chart_h)
            };

            for &child_idx in &node.children {
                let child = &nodes[child_idx];
                let (cx_, cy) = if self.horizontal_layout {
                    (chart_x + child.y * chart_w, chart_y + child.x * chart_h)
                } else {
                    (chart_x + child.x * chart_w, chart_y + child.y * chart_h)
                };

                // Animate link drawing
                let link_progress = ((progress - node.depth as f64 * 0.15) / 0.5).clamp(0.0, 1.0);
                if link_progress <= 0.0 {
                    continue;
                }

                // Gradient color based on depth
                let start_color = get_color(node.depth);
                let end_color = get_color(child.depth);

                // Draw smooth bezier curve
                if self.show_smooth_curves {
                    self.draw_bezier_link(
                        cx, px, py, cx_, cy,
                        self.horizontal_layout,
                        link_progress,
                        start_color,
                        end_color,
                    );
                } else {
                    // Draw straight elbow
                    self.draw_elbow_link(
                        cx, px, py, cx_, cy,
                        self.horizontal_layout,
                        link_progress,
                        start_color,
                    );
                }
            }
        }

        // Draw nodes
        for node in &nodes {
            // Stagger node appearance by depth
            let node_progress = ((progress - node.depth as f64 * 0.1) / 0.3).clamp(0.0, 1.0);
            if node_progress <= 0.0 {
                continue;
            }

            let (x, y) = if self.horizontal_layout {
                (chart_x + node.y * chart_w, chart_y + node.x * chart_h)
            } else {
                (chart_x + node.x * chart_w, chart_y + node.y * chart_h)
            };

            let color = get_color(node.depth);

            // Node size with animation
            let base_radius = if node.children.is_empty() { 6.0 } else { 10.0 };
            let radius = base_radius * node_progress;

            // Draw outer ring for internal nodes
            if !node.children.is_empty() {
                self.draw_point.color = vec4(color.x * 0.7, color.y * 0.7, color.z * 0.7, 0.5);
                self.draw_point.draw_point(cx, dvec2(x, y), radius * 1.5);
            }

            // Draw node circle
            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(x, y), radius);

            // Draw inner highlight
            self.draw_point.color = vec4(1.0, 1.0, 1.0, 0.4);
            self.draw_point.draw_point(cx, dvec2(x - radius * 0.2, y - radius * 0.2), radius * 0.4);
        }
    }

    fn draw_bezier_link(
        &mut self,
        cx: &mut Cx2d,
        x1: f64, y1: f64,
        x2: f64, y2: f64,
        horizontal: bool,
        progress: f64,
        start_color: Vec4,
        end_color: Vec4,
    ) {
        let segments = 24;
        let draw_segments = ((segments as f64 * progress) as usize).max(1);

        for i in 0..draw_segments {
            let t1 = i as f64 / segments as f64;
            let t2 = (i + 1) as f64 / segments as f64;

            // Cubic bezier control points for smooth S-curve
            let (ctrl1x, ctrl1y, ctrl2x, ctrl2y) = if horizontal {
                let mid_x = (x1 + x2) / 2.0;
                (mid_x, y1, mid_x, y2)
            } else {
                let mid_y = (y1 + y2) / 2.0;
                (x1, mid_y, x2, mid_y)
            };

            let p1 = self.cubic_bezier(x1, y1, ctrl1x, ctrl1y, ctrl2x, ctrl2y, x2, y2, t1);
            let p2 = self.cubic_bezier(x1, y1, ctrl1x, ctrl1y, ctrl2x, ctrl2y, x2, y2, t2);

            // Interpolate color along the curve
            let color = vec4(
                start_color.x + (end_color.x - start_color.x) * t1 as f32,
                start_color.y + (end_color.y - start_color.y) * t1 as f32,
                start_color.z + (end_color.z - start_color.z) * t1 as f32,
                0.7,
            );

            self.draw_line.color = color;
            self.draw_line.draw_line(cx, p1, p2, 2.0);
        }
    }

    fn draw_elbow_link(
        &mut self,
        cx: &mut Cx2d,
        x1: f64, y1: f64,
        x2: f64, y2: f64,
        horizontal: bool,
        progress: f64,
        color: Vec4,
    ) {
        self.draw_line.color = vec4(color.x, color.y, color.z, 0.6);

        if horizontal {
            let mid_x = (x1 + x2) / 2.0;
            let animated_mid = x1 + (mid_x - x1) * progress.min(0.5) * 2.0;
            let animated_end = if progress > 0.5 {
                mid_x + (x2 - mid_x) * (progress - 0.5) * 2.0
            } else {
                mid_x
            };

            // Horizontal segment
            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(animated_mid, y1), 2.0);
            if progress > 0.5 {
                // Vertical segment
                let vert_progress = (progress - 0.5) * 2.0;
                let animated_y = y1 + (y2 - y1) * vert_progress;
                self.draw_line.draw_line(cx, dvec2(mid_x, y1), dvec2(mid_x, animated_y), 2.0);
                // Final horizontal
                if progress > 0.75 {
                    self.draw_line.draw_line(cx, dvec2(mid_x, y2), dvec2(animated_end, y2), 2.0);
                }
            }
        } else {
            let mid_y = (y1 + y2) / 2.0;
            let animated_mid = y1 + (mid_y - y1) * progress.min(0.5) * 2.0;

            // Vertical segment
            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(x1, animated_mid), 2.0);
            if progress > 0.5 {
                // Horizontal segment
                let horiz_progress = (progress - 0.5) * 2.0;
                let animated_x = x1 + (x2 - x1) * horiz_progress;
                self.draw_line.draw_line(cx, dvec2(x1, mid_y), dvec2(animated_x, mid_y), 2.0);
                // Final vertical
                if progress > 0.75 {
                    let final_progress = (progress - 0.75) * 4.0;
                    let animated_y2 = mid_y + (y2 - mid_y) * final_progress;
                    self.draw_line.draw_line(cx, dvec2(x2, mid_y), dvec2(x2, animated_y2), 2.0);
                }
            }
        }
    }

    fn cubic_bezier(&self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, t: f64) -> DVec2 {
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        let t2 = t * t;
        let t3 = t2 * t;

        let x = mt3 * x0 + 3.0 * mt2 * t * x1 + 3.0 * mt * t2 * x2 + t3 * x3;
        let y = mt3 * y0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * y3;

        dvec2(x, y)
    }

    /// Initialize with D3 flare-style software package hierarchy (tidy tree layout)
    pub fn initialize_flare_data(&mut self) {
        // D3 flare visualization toolkit hierarchy
        self.nodes = vec![
            // Root (index 0)
            TreeNode { name: "flare".into(), children: vec![1, 2, 3, 4, 5], x: 0.0, y: 0.0, depth: 0 },

            // Level 1: Main categories
            TreeNode { name: "analytics".into(), children: vec![6, 7, 8, 9], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "animate".into(), children: vec![10, 11, 12, 13], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "data".into(), children: vec![14, 15, 16, 17], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "display".into(), children: vec![18, 19, 20, 21], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "vis".into(), children: vec![22, 23, 24], x: 0.0, y: 0.0, depth: 1 },

            // Level 2 under analytics
            TreeNode { name: "AgglomerativeCluster".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "CommunityStructure".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "HierarchicalCluster".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "MergeEdge".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under animate
            TreeNode { name: "Easing".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "FunctionSequence".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "Tween".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "Transitioner".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under data
            TreeNode { name: "DataField".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "DataSchema".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "DataSource".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "DataUtil".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under display
            TreeNode { name: "DirtySprite".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "LineSprite".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "RectSprite".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "TextSprite".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under vis
            TreeNode { name: "ScaleBinding".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "Tree".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "TreeBuilder".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
        ];

        self.horizontal_layout = true;
        self.show_smooth_curves = true;
        self.compute_layout();
        self.initialized = true;
    }

    /// Initialize with D3 cluster layout (dendrogram - all leaves at same depth)
    pub fn initialize_cluster_data(&mut self) {
        // Same flare hierarchy but will use cluster layout (all leaves aligned)
        self.nodes = vec![
            // Root (index 0)
            TreeNode { name: "flare".into(), children: vec![1, 2, 3], x: 0.0, y: 0.0, depth: 0 },

            // Level 1
            TreeNode { name: "analytics".into(), children: vec![4, 5, 6], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "animate".into(), children: vec![7, 8], x: 0.0, y: 0.0, depth: 1 },
            TreeNode { name: "data".into(), children: vec![9, 10, 11], x: 0.0, y: 0.0, depth: 1 },

            // Level 2 under analytics (with sub-children)
            TreeNode { name: "cluster".into(), children: vec![12, 13], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "graph".into(), children: vec![14, 15], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "optimization".into(), children: vec![16], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under animate
            TreeNode { name: "interpolate".into(), children: vec![17, 18], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "Scheduler".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 2 under data
            TreeNode { name: "converters".into(), children: vec![19, 20], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "DataField".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },
            TreeNode { name: "DataUtil".into(), children: vec![], x: 0.0, y: 0.0, depth: 2 },

            // Level 3 leaves
            TreeNode { name: "AgglomerativeCluster".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "HierarchicalCluster".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "BetweennessCentrality".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "SpanningTree".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "AspectRatioBanker".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "ArrayInterpolator".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "NumberInterpolator".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "JSONConverter".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
            TreeNode { name: "Converters".into(), children: vec![], x: 0.0, y: 0.0, depth: 3 },
        ];

        self.horizontal_layout = true;
        self.show_smooth_curves = true;
        self.compute_layout();
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl TreeChartWidgetRef {
    pub fn initialize_flare_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_flare_data();
            inner.replay_animation(cx);
        }
    }

    pub fn initialize_cluster_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_cluster_data();
            inner.replay_animation(cx);
        }
    }
}
