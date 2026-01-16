//! Treemap Widget
//!
//! GPU-accelerated hierarchical visualization with animated reveal.
//! Features smooth gradients, rounded corners, and staggered depth animation.

use makepad_widgets::*;
use makepad_d3::layout::hierarchy::{HierarchyNode, TreemapLayout, TilingMethod};
use super::draw_primitives::{DrawChartLine, DrawTriangle, DrawPoint};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawPoint;

    pub TreemapWidget = {{TreemapWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TreemapWidget {
    #[redraw]
    #[live]
    draw_rect: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    tree: Option<HierarchyNode<TreeNode>>,

    #[rust]
    positioned_tree: Option<HierarchyNode<TreeNode>>,

    #[rust]
    colors: Vec<Vec4>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    offset_x: f64,

    #[rust]
    offset_y: f64,

    #[rust]
    hovered_node: Option<(f64, f64, f64, f64)>,

    #[rust(true)]
    show_borders: bool,

    #[rust(true)]
    rounded_corners: bool,

    #[rust]
    node_count: usize,
}

#[derive(Clone)]
struct TreeNode {
    name: String,
    color_index: usize,
    depth: usize,
    leaf_index: usize,
}

impl Widget for TreemapWidget {
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
            self.chart_rect = rect;

            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_treemap(cx, rect);
        }

        DrawStep::done()
    }
}

impl TreemapWidget {
    fn initialize_data(&mut self) {
        // Create a hierarchical dataset (file system-like structure)
        let mut root = HierarchyNode::new(
            TreeNode { name: "root".to_string(), color_index: 0, depth: 0, leaf_index: 0 },
            0.0,
        );

        // Category 1: Technology
        let mut tech = HierarchyNode::new(
            TreeNode { name: "Technology".to_string(), color_index: 0, depth: 1, leaf_index: 0 },
            0.0,
        );
        tech.add_child(HierarchyNode::new(
            TreeNode { name: "Software".to_string(), color_index: 0, depth: 2, leaf_index: 0 },
            120.0,
        ));
        tech.add_child(HierarchyNode::new(
            TreeNode { name: "Hardware".to_string(), color_index: 0, depth: 2, leaf_index: 1 },
            80.0,
        ));
        tech.add_child(HierarchyNode::new(
            TreeNode { name: "Cloud".to_string(), color_index: 0, depth: 2, leaf_index: 2 },
            95.0,
        ));
        tech.add_child(HierarchyNode::new(
            TreeNode { name: "AI/ML".to_string(), color_index: 0, depth: 2, leaf_index: 3 },
            150.0,
        ));

        // Category 2: Healthcare
        let mut health = HierarchyNode::new(
            TreeNode { name: "Healthcare".to_string(), color_index: 1, depth: 1, leaf_index: 0 },
            0.0,
        );
        health.add_child(HierarchyNode::new(
            TreeNode { name: "Pharma".to_string(), color_index: 1, depth: 2, leaf_index: 4 },
            90.0,
        ));
        health.add_child(HierarchyNode::new(
            TreeNode { name: "Biotech".to_string(), color_index: 1, depth: 2, leaf_index: 5 },
            70.0,
        ));
        health.add_child(HierarchyNode::new(
            TreeNode { name: "Devices".to_string(), color_index: 1, depth: 2, leaf_index: 6 },
            45.0,
        ));

        // Category 3: Finance
        let mut finance = HierarchyNode::new(
            TreeNode { name: "Finance".to_string(), color_index: 2, depth: 1, leaf_index: 0 },
            0.0,
        );
        finance.add_child(HierarchyNode::new(
            TreeNode { name: "Banking".to_string(), color_index: 2, depth: 2, leaf_index: 7 },
            100.0,
        ));
        finance.add_child(HierarchyNode::new(
            TreeNode { name: "Insurance".to_string(), color_index: 2, depth: 2, leaf_index: 8 },
            60.0,
        ));
        finance.add_child(HierarchyNode::new(
            TreeNode { name: "Fintech".to_string(), color_index: 2, depth: 2, leaf_index: 9 },
            85.0,
        ));

        // Category 4: Energy
        let mut energy = HierarchyNode::new(
            TreeNode { name: "Energy".to_string(), color_index: 3, depth: 1, leaf_index: 0 },
            0.0,
        );
        energy.add_child(HierarchyNode::new(
            TreeNode { name: "Solar".to_string(), color_index: 3, depth: 2, leaf_index: 10 },
            55.0,
        ));
        energy.add_child(HierarchyNode::new(
            TreeNode { name: "Wind".to_string(), color_index: 3, depth: 2, leaf_index: 11 },
            40.0,
        ));
        energy.add_child(HierarchyNode::new(
            TreeNode { name: "Oil & Gas".to_string(), color_index: 3, depth: 2, leaf_index: 12 },
            75.0,
        ));

        // Category 5: Consumer
        let mut consumer = HierarchyNode::new(
            TreeNode { name: "Consumer".to_string(), color_index: 4, depth: 1, leaf_index: 0 },
            0.0,
        );
        consumer.add_child(HierarchyNode::new(
            TreeNode { name: "Retail".to_string(), color_index: 4, depth: 2, leaf_index: 13 },
            65.0,
        ));
        consumer.add_child(HierarchyNode::new(
            TreeNode { name: "Media".to_string(), color_index: 4, depth: 2, leaf_index: 14 },
            50.0,
        ));

        root.add_child(tech);
        root.add_child(health);
        root.add_child(finance);
        root.add_child(energy);
        root.add_child(consumer);

        self.tree = Some(root);
        self.node_count = 15; // Total leaf nodes

        // Vibrant color palette
        self.colors = vec![
            vec4(0.26, 0.52, 0.96, 1.0), // Blue (Tech)
            vec4(0.20, 0.78, 0.50, 1.0), // Green (Health)
            vec4(1.0, 0.76, 0.03, 1.0),  // Gold (Finance)
            vec4(0.92, 0.36, 0.32, 1.0), // Red (Energy)
            vec4(0.61, 0.35, 0.80, 1.0), // Purple (Consumer)
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1400.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_treemap(&mut self, cx: &mut Cx2d, rect: Rect) {
        let tree = match &self.tree {
            Some(t) => t,
            None => return,
        };

        let padding = 10.0;
        self.offset_x = rect.pos.x + padding;
        self.offset_y = rect.pos.y + padding;

        let layout = TreemapLayout::new()
            .size(
                (rect.size.x - padding * 2.0) as f64,
                (rect.size.y - padding * 2.0) as f64,
            )
            .padding(4.0)
            .padding_top(0.0)
            .tiling(TilingMethod::Squarify);

        let positioned = layout.layout(tree);
        self.positioned_tree = Some(positioned.clone());

        let progress = self.animator.get_progress();

        // Draw background
        self.draw_rect.color = vec4(0.15, 0.15, 0.18, 1.0);
        self.draw_rect.disable_gradient();
        let bg_p1 = dvec2(self.offset_x - 5.0, self.offset_y - 5.0);
        let bg_p2 = dvec2(self.offset_x + rect.size.x - padding * 2.0 + 5.0, self.offset_y - 5.0);
        let bg_p3 = dvec2(self.offset_x + rect.size.x - padding * 2.0 + 5.0, self.offset_y + rect.size.y - padding * 2.0 + 5.0);
        let bg_p4 = dvec2(self.offset_x - 5.0, self.offset_y + rect.size.y - padding * 2.0 + 5.0);
        self.draw_rect.draw_triangle(cx, bg_p1, bg_p2, bg_p3);
        self.draw_rect.draw_triangle(cx, bg_p1, bg_p3, bg_p4);

        // Draw all leaf nodes with animation
        self.draw_node(cx, &positioned, self.offset_x, self.offset_y, progress);
    }

    fn draw_node(&mut self, cx: &mut Cx2d, node: &HierarchyNode<TreeNode>, offset_x: f64, offset_y: f64, progress: f64) {
        if node.is_leaf() && node.width > 0.0 && node.rect_height > 0.0 {
            // Stagger animation by leaf index
            let leaf_idx = node.data.leaf_index;
            let node_delay = leaf_idx as f64 * 0.04;
            let node_progress = ((progress - node_delay) / 0.5).clamp(0.0, 1.0);

            if node_progress <= 0.0 {
                return;
            }

            let color_idx = node.data.color_index % self.colors.len();
            let mut base_color = self.colors[color_idx];

            // Calculate node rectangle
            let full_x = offset_x + node.x;
            let full_y = offset_y + node.y;
            let full_w = node.width;
            let full_h = node.rect_height;

            // Animate from center
            let w = full_w * node_progress;
            let h = full_h * node_progress;
            let x = full_x + (full_w - w) / 2.0;
            let y = full_y + (full_h - h) / 2.0;

            // Check if this node is hovered
            let is_hovered = self.hovered_node.map_or(false, |hover| {
                (hover.0 - full_x).abs() < 0.1 &&
                (hover.1 - full_y).abs() < 0.1 &&
                (hover.2 - full_w).abs() < 0.1 &&
                (hover.3 - full_h).abs() < 0.1
            });

            // Brighten hovered node
            if is_hovered {
                base_color = vec4(
                    (base_color.x + 0.18).min(1.0),
                    (base_color.y + 0.18).min(1.0),
                    (base_color.z + 0.18).min(1.0),
                    base_color.w,
                );
            }

            // Draw the rectangle with gradient
            self.draw_cell(cx, x, y, w, h, base_color, node_progress, is_hovered);
        }

        // Recursively draw children
        for child in &node.children {
            self.draw_node(cx, child, offset_x, offset_y, progress);
        }
    }

    fn draw_cell(
        &mut self,
        cx: &mut Cx2d,
        x: f64, y: f64,
        w: f64, h: f64,
        color: Vec4,
        _progress: f64,
        is_hovered: bool,
    ) {
        // Flat solid color
        self.draw_rect.color = color;
        self.draw_rect.disable_gradient();

        let p1 = dvec2(x, y);
        let p2 = dvec2(x + w, y);
        let p3 = dvec2(x + w, y + h);
        let p4 = dvec2(x, y + h);

        self.draw_rect.draw_triangle(cx, p1, p2, p3);
        self.draw_rect.draw_triangle(cx, p1, p3, p4);

        // Draw borders
        if self.show_borders {
            let border_color = if is_hovered {
                vec4(1.0, 1.0, 1.0, 0.8)
            } else {
                vec4(0.1, 0.1, 0.12, 0.8)
            };
            let border_width = if is_hovered { 2.0 } else { 1.0 };

            self.draw_line.color = border_color;
            self.draw_line.draw_line(cx, p1, p2, border_width);
            self.draw_line.draw_line(cx, p2, p3, border_width);
            self.draw_line.draw_line(cx, p3, p4, border_width);
            self.draw_line.draw_line(cx, p4, p1, border_width);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_node;
        self.hovered_node = self.find_node_at(pos);

        if old_hovered != self.hovered_node {
            self.redraw(cx);
        }
    }

    fn find_node_at(&self, pos: DVec2) -> Option<(f64, f64, f64, f64)> {
        let tree = self.positioned_tree.as_ref()?;
        self.find_leaf_at(tree, pos, self.offset_x, self.offset_y)
    }

    fn find_leaf_at(&self, node: &HierarchyNode<TreeNode>, pos: DVec2, offset_x: f64, offset_y: f64) -> Option<(f64, f64, f64, f64)> {
        if node.is_leaf() && node.width > 0.0 && node.rect_height > 0.0 {
            let x = offset_x + node.x;
            let y = offset_y + node.y;
            let w = node.width;
            let h = node.rect_height;

            if pos.x >= x && pos.x <= x + w && pos.y >= y && pos.y <= y + h {
                return Some((x, y, w, h));
            }
        }

        for child in &node.children {
            if let Some(found) = self.find_leaf_at(child, pos, offset_x, offset_y) {
                return Some(found);
            }
        }
        None
    }

    /// Initialize with D3 flare-style software package hierarchy data
    pub fn initialize_flare_data(&mut self) {
        let mut root = HierarchyNode::new(
            TreeNode { name: "flare".to_string(), color_index: 0, depth: 0, leaf_index: 0 },
            0.0,
        );

        let mut leaf_idx = 0usize;

        // analytics
        let mut analytics = HierarchyNode::new(
            TreeNode { name: "analytics".to_string(), color_index: 0, depth: 1, leaf_index: 0 },
            0.0,
        );
        analytics.add_child(HierarchyNode::new(TreeNode { name: "AgglomerativeCluster".to_string(), color_index: 0, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3938.0));
        analytics.add_child(HierarchyNode::new(TreeNode { name: "CommunityStructure".to_string(), color_index: 0, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3812.0));
        analytics.add_child(HierarchyNode::new(TreeNode { name: "HierarchicalCluster".to_string(), color_index: 0, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 6714.0));
        analytics.add_child(HierarchyNode::new(TreeNode { name: "MergeEdge".to_string(), color_index: 0, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 743.0));
        root.add_child(analytics);

        // animate
        let mut animate = HierarchyNode::new(
            TreeNode { name: "animate".to_string(), color_index: 1, depth: 1, leaf_index: 0 },
            0.0,
        );
        animate.add_child(HierarchyNode::new(TreeNode { name: "Easing".to_string(), color_index: 1, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 17010.0));
        animate.add_child(HierarchyNode::new(TreeNode { name: "FunctionSequence".to_string(), color_index: 1, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 5842.0));
        animate.add_child(HierarchyNode::new(TreeNode { name: "Tween".to_string(), color_index: 1, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 6006.0));
        animate.add_child(HierarchyNode::new(TreeNode { name: "Transitioner".to_string(), color_index: 1, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 19975.0));
        root.add_child(animate);

        // data
        let mut data = HierarchyNode::new(
            TreeNode { name: "data".to_string(), color_index: 2, depth: 1, leaf_index: 0 },
            0.0,
        );
        data.add_child(HierarchyNode::new(TreeNode { name: "DataField".to_string(), color_index: 2, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 1759.0));
        data.add_child(HierarchyNode::new(TreeNode { name: "DataSchema".to_string(), color_index: 2, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 2165.0));
        data.add_child(HierarchyNode::new(TreeNode { name: "DataSet".to_string(), color_index: 2, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 586.0));
        data.add_child(HierarchyNode::new(TreeNode { name: "DataSource".to_string(), color_index: 2, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3331.0));
        data.add_child(HierarchyNode::new(TreeNode { name: "DataUtil".to_string(), color_index: 2, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3322.0));
        root.add_child(data);

        // display
        let mut display = HierarchyNode::new(
            TreeNode { name: "display".to_string(), color_index: 3, depth: 1, leaf_index: 0 },
            0.0,
        );
        display.add_child(HierarchyNode::new(TreeNode { name: "DirtySprite".to_string(), color_index: 3, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 8833.0));
        display.add_child(HierarchyNode::new(TreeNode { name: "LineSprite".to_string(), color_index: 3, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 1732.0));
        display.add_child(HierarchyNode::new(TreeNode { name: "RectSprite".to_string(), color_index: 3, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3623.0));
        display.add_child(HierarchyNode::new(TreeNode { name: "TextSprite".to_string(), color_index: 3, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 10066.0));
        root.add_child(display);

        // scale
        let mut scale = HierarchyNode::new(
            TreeNode { name: "scale".to_string(), color_index: 4, depth: 1, leaf_index: 0 },
            0.0,
        );
        scale.add_child(HierarchyNode::new(TreeNode { name: "LinearScale".to_string(), color_index: 4, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 1316.0));
        scale.add_child(HierarchyNode::new(TreeNode { name: "LogScale".to_string(), color_index: 4, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3151.0));
        scale.add_child(HierarchyNode::new(TreeNode { name: "OrdinalScale".to_string(), color_index: 4, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 3770.0));
        scale.add_child(HierarchyNode::new(TreeNode { name: "TimeScale".to_string(), color_index: 4, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 5833.0));
        root.add_child(scale);

        // vis
        let mut vis = HierarchyNode::new(
            TreeNode { name: "vis".to_string(), color_index: 5, depth: 1, leaf_index: 0 },
            0.0,
        );
        vis.add_child(HierarchyNode::new(TreeNode { name: "ScaleBinding".to_string(), color_index: 5, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 11275.0));
        vis.add_child(HierarchyNode::new(TreeNode { name: "Tree".to_string(), color_index: 5, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 7147.0));
        vis.add_child(HierarchyNode::new(TreeNode { name: "TreeBuilder".to_string(), color_index: 5, depth: 2, leaf_index: { leaf_idx += 1; leaf_idx - 1 } }, 9930.0));
        root.add_child(vis);

        self.tree = Some(root);
        self.node_count = leaf_idx;

        // D3 schemeTableau10 colors
        self.colors = vec![
            vec4(0.26, 0.52, 0.96, 1.0), // Blue - analytics
            vec4(1.0, 0.50, 0.05, 1.0),  // Orange - animate
            vec4(0.17, 0.63, 0.17, 1.0), // Green - data
            vec4(0.84, 0.15, 0.16, 1.0), // Red - display
            vec4(0.58, 0.40, 0.74, 1.0), // Purple - scale
            vec4(0.55, 0.34, 0.29, 1.0), // Brown - vis
        ];
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl TreemapWidgetRef {
    pub fn initialize_flare_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_flare_data();
            inner.replay_animation(cx);
        }
    }
}
