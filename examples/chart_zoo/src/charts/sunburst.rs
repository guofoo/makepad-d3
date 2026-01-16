//! Sunburst Widget
//!
//! Radial hierarchical visualization using concentric arcs.
//! GPU-accelerated with proper animation support.

use makepad_widgets::*;
use makepad_d3::layout::hierarchy::HierarchyNode;
use std::f64::consts::PI;
use super::draw_primitives::DrawArc;
use super::animation::{ChartAnimator, EasingType, get_color, lighten, darken};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawArc;

    pub SunburstWidget = {{SunburstWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SunburstWidget {
    #[live]
    #[deref]
    view: View,

    #[redraw]
    #[live]
    draw_arc: DrawArc,

    #[walk]
    walk: Walk,

    #[rust]
    tree: Option<HierarchyNode<SunburstNode>>,

    #[rust]
    colors: Vec<Vec4>,

    #[rust]
    initialized: bool,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    area: Area,

    #[rust]
    center_x: f64,

    #[rust]
    center_y: f64,

    #[rust]
    max_radius: f64,

    #[rust]
    max_depth: usize,

    #[rust]
    hovered_arc: Option<(f64, f64, usize)>, // start_angle, end_angle, depth
}

#[derive(Clone)]
struct SunburstNode {
    name: String,
    color_index: usize,
}

impl Widget for SunburstWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

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
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
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
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_sunburst(cx, rect);
        }

        DrawStep::done()
    }
}

impl SunburstWidget {
    fn initialize_data(&mut self) {
        // Create hierarchical data (programming languages by paradigm)
        let mut root = HierarchyNode::new(
            SunburstNode { name: "Languages".to_string(), color_index: 0 },
            0.0,
        );

        // Object-Oriented
        let mut oop = HierarchyNode::new(
            SunburstNode { name: "OOP".to_string(), color_index: 0 },
            0.0,
        );
        oop.add_child(HierarchyNode::new(
            SunburstNode { name: "Java".to_string(), color_index: 0 },
            35.0,
        ));
        oop.add_child(HierarchyNode::new(
            SunburstNode { name: "C#".to_string(), color_index: 0 },
            25.0,
        ));
        oop.add_child(HierarchyNode::new(
            SunburstNode { name: "C++".to_string(), color_index: 0 },
            30.0,
        ));

        // Functional
        let mut functional = HierarchyNode::new(
            SunburstNode { name: "Functional".to_string(), color_index: 1 },
            0.0,
        );
        functional.add_child(HierarchyNode::new(
            SunburstNode { name: "Haskell".to_string(), color_index: 1 },
            15.0,
        ));
        functional.add_child(HierarchyNode::new(
            SunburstNode { name: "Scala".to_string(), color_index: 1 },
            18.0,
        ));
        functional.add_child(HierarchyNode::new(
            SunburstNode { name: "F#".to_string(), color_index: 1 },
            12.0,
        ));
        functional.add_child(HierarchyNode::new(
            SunburstNode { name: "Clojure".to_string(), color_index: 1 },
            10.0,
        ));

        // Systems
        let mut systems = HierarchyNode::new(
            SunburstNode { name: "Systems".to_string(), color_index: 2 },
            0.0,
        );
        systems.add_child(HierarchyNode::new(
            SunburstNode { name: "Rust".to_string(), color_index: 2 },
            28.0,
        ));
        systems.add_child(HierarchyNode::new(
            SunburstNode { name: "C".to_string(), color_index: 2 },
            40.0,
        ));
        systems.add_child(HierarchyNode::new(
            SunburstNode { name: "Go".to_string(), color_index: 2 },
            22.0,
        ));

        // Scripting
        let mut scripting = HierarchyNode::new(
            SunburstNode { name: "Scripting".to_string(), color_index: 3 },
            0.0,
        );
        scripting.add_child(HierarchyNode::new(
            SunburstNode { name: "Python".to_string(), color_index: 3 },
            45.0,
        ));
        scripting.add_child(HierarchyNode::new(
            SunburstNode { name: "JavaScript".to_string(), color_index: 3 },
            50.0,
        ));
        scripting.add_child(HierarchyNode::new(
            SunburstNode { name: "Ruby".to_string(), color_index: 3 },
            15.0,
        ));
        scripting.add_child(HierarchyNode::new(
            SunburstNode { name: "PHP".to_string(), color_index: 3 },
            20.0,
        ));

        root.add_child(oop);
        root.add_child(functional);
        root.add_child(systems);
        root.add_child(scripting);

        self.tree = Some(root);

        // Vibrant color palette
        self.colors = vec![
            vec4(0.95, 0.50, 0.40, 1.0), // Coral (OOP)
            vec4(0.45, 0.75, 0.55, 1.0), // Green (Functional)
            vec4(0.40, 0.60, 0.95, 1.0), // Blue (Systems)
            vec4(0.95, 0.75, 0.35, 1.0), // Orange (Scripting)
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0) // 1 second animation
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
        self.redraw(cx);
    }

    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn draw_sunburst(&mut self, cx: &mut Cx2d, rect: Rect) {
        // Sum values first
        if let Some(ref mut tree) = self.tree {
            tree.sum();
            tree.each_before();
        }

        self.center_x = (rect.pos.x + rect.size.x / 2.0) as f64;
        self.center_y = (rect.pos.y + rect.size.y / 2.0) as f64;
        self.max_radius = (rect.size.x.min(rect.size.y) / 2.0 - 15.0) as f64;

        // Find max depth (needs immutable borrow)
        self.max_depth = if let Some(ref tree) = self.tree {
            Self::find_max_depth_static(tree)
        } else {
            return;
        };

        // Clone tree for drawing to avoid borrow conflicts
        let tree_clone = if let Some(ref tree) = self.tree {
            tree.clone()
        } else {
            return;
        };

        // Get animation progress
        let progress = self.animator.get_progress();

        // Clone colors to avoid borrow issues
        let colors = self.colors.clone();
        let hovered_arc = self.hovered_arc;

        // Draw arcs
        self.draw_node_arcs(
            cx,
            &tree_clone,
            self.center_x,
            self.center_y,
            self.max_radius,
            self.max_depth,
            0.0,
            2.0 * PI,
            0,
            progress,
            &colors,
            hovered_arc,
        );
    }

    fn find_max_depth_static(node: &HierarchyNode<SunburstNode>) -> usize {
        if node.children.is_empty() {
            return 0;
        }
        1 + node.children.iter()
            .map(|c| Self::find_max_depth_static(c))
            .max()
            .unwrap_or(0)
    }

    fn draw_node_arcs(
        &mut self,
        cx: &mut Cx2d,
        node: &HierarchyNode<SunburstNode>,
        cx_: f64,
        cy: f64,
        max_radius: f64,
        max_depth: usize,
        start_angle: f64,
        end_angle: f64,
        depth: usize,
        progress: f64,
        colors: &[Vec4],
        hovered_arc: Option<(f64, f64, usize)>,
    ) {
        if depth == 0 && !node.children.is_empty() {
            // Skip root, just draw children
            let total_value = node.value;
            let mut angle = start_angle;

            for child in &node.children {
                let child_angle = (child.value / total_value) * (end_angle - start_angle);
                self.draw_node_arcs(
                    cx,
                    child,
                    cx_, cy,
                    max_radius,
                    max_depth,
                    angle,
                    angle + child_angle,
                    depth + 1,
                    progress,
                    colors,
                    hovered_arc,
                );
                angle += child_angle;
            }
            return;
        }

        // Calculate ring dimensions
        let ring_width = max_radius / (max_depth + 1) as f64;
        let inner_radius = (depth as f64 - 1.0) * ring_width;
        let outer_radius = depth as f64 * ring_width;

        // Apply animation - grow outward from center
        let animated_outer = inner_radius + (outer_radius - inner_radius) * progress;

        if animated_outer > inner_radius && (end_angle - start_angle) > 0.01 {
            // Check if this arc is hovered
            let is_hovered = hovered_arc.map_or(false, |(hs, he, hd)| {
                (hs - start_angle).abs() < 0.01 && (he - end_angle).abs() < 0.01 && hd == depth
            });

            // Get color
            let color_idx = node.data.color_index % colors.len();
            let base_color = colors[color_idx];

            // Vary brightness by depth, more for hovered
            let brightness = if is_hovered {
                1.15 - (depth as f32 - 1.0) * 0.1
            } else {
                1.0 - (depth as f32 - 1.0) * 0.15
            };
            let color = vec4(
                (base_color.x * brightness).min(1.0),
                (base_color.y * brightness).min(1.0),
                (base_color.z * brightness).min(1.0),
                base_color.w,
            );

            // Gradient colors - lighter inside, darker outside
            let inner_color = vec4(
                (color.x + 0.15).min(1.0),
                (color.y + 0.15).min(1.0),
                (color.z + 0.15).min(1.0),
                color.w,
            );
            let outer_color = vec4(
                (color.x - 0.1).max(0.0),
                (color.y - 0.1).max(0.0),
                (color.z - 0.1).max(0.0),
                color.w,
            );

            // Draw the arc (slightly larger outer radius for hover)
            // Adjust angles to start from top (-PI/2)
            let adjusted_start = start_angle - PI / 2.0;
            let sweep = (end_angle - start_angle - 0.01) * progress; // Animate sweep
            let hover_scale = if is_hovered { 1.05 } else { 1.0 };

            self.draw_arc.set_arc(adjusted_start, sweep, inner_radius, animated_outer * hover_scale);
            self.draw_arc.set_radial_gradient(inner_color, outer_color);
            self.draw_arc.draw_arc(cx, dvec2(cx_, cy), animated_outer * hover_scale);
        }

        // Draw children (only if animation has progressed enough)
        if progress > 0.3 && !node.children.is_empty() {
            let total_value = node.value;
            let mut angle = start_angle;

            // Stagger child animations
            let child_progress = ((progress - 0.3) / 0.7).clamp(0.0, 1.0);

            for child in &node.children {
                let child_angle = if total_value > 0.0 {
                    (child.value / total_value) * (end_angle - start_angle)
                } else {
                    0.0
                };

                self.draw_node_arcs(
                    cx,
                    child,
                    cx_, cy,
                    max_radius,
                    max_depth,
                    angle,
                    angle + child_angle,
                    depth + 1,
                    child_progress,
                    colors,
                    hovered_arc,
                );
                angle += child_angle;
            }
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_arc;
        self.hovered_arc = self.find_arc_at(pos);

        if old_hovered != self.hovered_arc {
            self.redraw(cx);
        }
    }

    fn find_arc_at(&self, pos: DVec2) -> Option<(f64, f64, usize)> {
        let dx = pos.x - self.center_x;
        let dy = pos.y - self.center_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > self.max_radius || self.max_depth == 0 {
            return None;
        }

        let ring_width = self.max_radius / (self.max_depth + 1) as f64;

        // Determine which ring (depth) based on distance
        let depth = ((dist / ring_width) + 1.0).floor() as usize;
        if depth == 0 || depth > self.max_depth {
            return None;
        }

        // Calculate angle (0 at top, clockwise)
        let mut angle = dy.atan2(dx) + PI / 2.0;
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        if angle > 2.0 * PI {
            angle -= 2.0 * PI;
        }

        // Find which arc contains this angle at this depth
        let tree = self.tree.as_ref()?;
        self.find_arc_in_tree(tree, angle, depth, 0.0, 2.0 * PI, 0)
    }

    fn find_arc_in_tree(
        &self,
        node: &HierarchyNode<SunburstNode>,
        target_angle: f64,
        target_depth: usize,
        start_angle: f64,
        end_angle: f64,
        current_depth: usize,
    ) -> Option<(f64, f64, usize)> {
        if current_depth == 0 && !node.children.is_empty() {
            // Root level - check children
            let total_value = node.value;
            let mut angle = start_angle;

            for child in &node.children {
                let child_angle = (child.value / total_value) * (end_angle - start_angle);
                if let Some(found) = self.find_arc_in_tree(
                    child,
                    target_angle,
                    target_depth,
                    angle,
                    angle + child_angle,
                    current_depth + 1,
                ) {
                    return Some(found);
                }
                angle += child_angle;
            }
            return None;
        }

        // Check if angle is within this arc's range
        if target_angle >= start_angle && target_angle < end_angle {
            if current_depth == target_depth {
                return Some((start_angle, end_angle, current_depth));
            }

            // Check children
            if !node.children.is_empty() {
                let total_value = node.value;
                let mut angle = start_angle;

                for child in &node.children {
                    let child_angle = if total_value > 0.0 {
                        (child.value / total_value) * (end_angle - start_angle)
                    } else {
                        0.0
                    };

                    if let Some(found) = self.find_arc_in_tree(
                        child,
                        target_angle,
                        target_depth,
                        angle,
                        angle + child_angle,
                        current_depth + 1,
                    ) {
                        return Some(found);
                    }
                    angle += child_angle;
                }
            }
        }
        None
    }

    /// Initialize with D3 flare-style software package hierarchy data
    pub fn initialize_flare_data(&mut self) {
        // Flare visualization toolkit hierarchy (D3 sunburst example)
        let mut root = HierarchyNode::new(
            SunburstNode { name: "flare".to_string(), color_index: 0 },
            0.0,
        );

        // analytics
        let mut analytics = HierarchyNode::new(
            SunburstNode { name: "analytics".to_string(), color_index: 0 },
            0.0,
        );
        let mut cluster = HierarchyNode::new(
            SunburstNode { name: "cluster".to_string(), color_index: 0 },
            0.0,
        );
        cluster.add_child(HierarchyNode::new(SunburstNode { name: "AgglomerativeCluster".to_string(), color_index: 0 }, 3938.0));
        cluster.add_child(HierarchyNode::new(SunburstNode { name: "CommunityStructure".to_string(), color_index: 0 }, 3812.0));
        cluster.add_child(HierarchyNode::new(SunburstNode { name: "HierarchicalCluster".to_string(), color_index: 0 }, 6714.0));
        cluster.add_child(HierarchyNode::new(SunburstNode { name: "MergeEdge".to_string(), color_index: 0 }, 743.0));
        analytics.add_child(cluster);
        let mut graph = HierarchyNode::new(
            SunburstNode { name: "graph".to_string(), color_index: 0 },
            0.0,
        );
        graph.add_child(HierarchyNode::new(SunburstNode { name: "BetweennessCentrality".to_string(), color_index: 0 }, 3534.0));
        graph.add_child(HierarchyNode::new(SunburstNode { name: "LinkDistance".to_string(), color_index: 0 }, 5731.0));
        graph.add_child(HierarchyNode::new(SunburstNode { name: "SpanningTree".to_string(), color_index: 0 }, 3416.0));
        analytics.add_child(graph);
        root.add_child(analytics);

        // animate
        let mut animate = HierarchyNode::new(
            SunburstNode { name: "animate".to_string(), color_index: 1 },
            0.0,
        );
        animate.add_child(HierarchyNode::new(SunburstNode { name: "Easing".to_string(), color_index: 1 }, 17010.0));
        animate.add_child(HierarchyNode::new(SunburstNode { name: "FunctionSequence".to_string(), color_index: 1 }, 5842.0));
        animate.add_child(HierarchyNode::new(SunburstNode { name: "Tween".to_string(), color_index: 1 }, 6006.0));
        animate.add_child(HierarchyNode::new(SunburstNode { name: "Transitioner".to_string(), color_index: 1 }, 19975.0));
        root.add_child(animate);

        // data
        let mut data = HierarchyNode::new(
            SunburstNode { name: "data".to_string(), color_index: 2 },
            0.0,
        );
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataField".to_string(), color_index: 2 }, 1759.0));
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataSchema".to_string(), color_index: 2 }, 2165.0));
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataSet".to_string(), color_index: 2 }, 586.0));
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataSource".to_string(), color_index: 2 }, 3331.0));
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataTable".to_string(), color_index: 2 }, 772.0));
        data.add_child(HierarchyNode::new(SunburstNode { name: "DataUtil".to_string(), color_index: 2 }, 3322.0));
        root.add_child(data);

        // display
        let mut display = HierarchyNode::new(
            SunburstNode { name: "display".to_string(), color_index: 3 },
            0.0,
        );
        display.add_child(HierarchyNode::new(SunburstNode { name: "DirtySprite".to_string(), color_index: 3 }, 8833.0));
        display.add_child(HierarchyNode::new(SunburstNode { name: "LineSprite".to_string(), color_index: 3 }, 1732.0));
        display.add_child(HierarchyNode::new(SunburstNode { name: "RectSprite".to_string(), color_index: 3 }, 3623.0));
        display.add_child(HierarchyNode::new(SunburstNode { name: "TextSprite".to_string(), color_index: 3 }, 10066.0));
        root.add_child(display);

        // vis (visualization)
        let mut vis = HierarchyNode::new(
            SunburstNode { name: "vis".to_string(), color_index: 4 },
            0.0,
        );
        vis.add_child(HierarchyNode::new(SunburstNode { name: "ScaleBinding".to_string(), color_index: 4 }, 11275.0));
        vis.add_child(HierarchyNode::new(SunburstNode { name: "Tree".to_string(), color_index: 4 }, 7147.0));
        vis.add_child(HierarchyNode::new(SunburstNode { name: "TreeBuilder".to_string(), color_index: 4 }, 9930.0));
        root.add_child(vis);

        self.tree = Some(root);

        // Rainbow color palette (d3.quantize(d3.interpolateRainbow, n))
        self.colors = vec![
            vec4(0.44, 0.19, 0.63, 1.0),  // Purple - analytics
            vec4(0.12, 0.47, 0.71, 1.0),  // Blue - animate
            vec4(0.20, 0.63, 0.17, 1.0),  // Green - data
            vec4(0.89, 0.47, 0.20, 1.0),  // Orange - display
            vec4(0.84, 0.15, 0.16, 1.0),  // Red - vis
        ];
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl SunburstWidgetRef {
    pub fn initialize_flare_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_flare_data();
            inner.replay_animation(cx);
        }
    }
}
