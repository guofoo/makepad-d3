//! Treemap layout algorithm
//!
//! Space-filling visualization for hierarchical data using nested rectangles.

use super::node::HierarchyNode;

/// Tiling method for treemap layout
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TilingMethod {
    /// Squarified tiling (default) - produces square-ish rectangles
    #[default]
    Squarify,
    /// Binary tiling - alternates horizontal/vertical splits
    Binary,
    /// Slice tiling - horizontal slices
    Slice,
    /// Dice tiling - vertical slices
    Dice,
    /// Slice-and-dice - alternates by depth
    SliceDice,
}

/// Treemap layout for hierarchical data
///
/// Creates space-filling visualizations where area represents value.
///
/// # Example
///
/// ```
/// use makepad_d3::layout::hierarchy::{HierarchyNode, TreemapLayout, TilingMethod};
///
/// let mut root = HierarchyNode::new("root", 0.0);
/// root.add_child(HierarchyNode::new("A", 30.0));
/// root.add_child(HierarchyNode::new("B", 20.0));
/// root.add_child(HierarchyNode::new("C", 50.0));
///
/// let layout = TreemapLayout::new()
///     .size(800.0, 600.0)
///     .padding(2.0)
///     .tiling(TilingMethod::Squarify);
///
/// let positioned = layout.layout(&root);
///
/// for node in positioned.iter() {
///     println!("{}: ({}, {}) {}x{}",
///         node.data, node.x, node.y, node.width, node.rect_height);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct TreemapLayout {
    /// Layout width
    width: f64,
    /// Layout height
    height: f64,
    /// Padding between siblings
    padding: f64,
    /// Padding at top for labels
    padding_top: f64,
    /// Padding around the root
    padding_outer: f64,
    /// Tiling method
    tiling: TilingMethod,
    /// Whether to round coordinates to pixels
    round: bool,
}

impl Default for TreemapLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl TreemapLayout {
    /// Create a new treemap layout
    pub fn new() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            padding: 0.0,
            padding_top: 0.0,
            padding_outer: 0.0,
            tiling: TilingMethod::Squarify,
            round: false,
        }
    }

    /// Set the layout size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set padding between siblings
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// Set padding at top (for labels)
    pub fn padding_top(mut self, padding: f64) -> Self {
        self.padding_top = padding.max(0.0);
        self
    }

    /// Set padding around root
    pub fn padding_outer(mut self, padding: f64) -> Self {
        self.padding_outer = padding.max(0.0);
        self
    }

    /// Set the tiling method
    pub fn tiling(mut self, method: TilingMethod) -> Self {
        self.tiling = method;
        self
    }

    /// Enable rounding to whole pixels
    pub fn round(mut self, round: bool) -> Self {
        self.round = round;
        self
    }

    /// Apply the layout to a hierarchy
    pub fn layout<T: Clone>(&self, root: &HierarchyNode<T>) -> HierarchyNode<T> {
        let mut tree = root.clone_tree();

        // Sum values if not already done
        tree.sum();
        tree.each_before();

        // Set root dimensions
        tree.x = self.padding_outer;
        tree.y = self.padding_outer;
        tree.width = self.width - 2.0 * self.padding_outer;
        tree.rect_height = self.height - 2.0 * self.padding_outer;

        // Apply tiling recursively
        self.tile_node(&mut tree);

        // Round if requested
        if self.round {
            self.round_coords(&mut tree);
        }

        tree
    }

    /// Tile a node and its children
    fn tile_node<T>(&self, node: &mut HierarchyNode<T>) {
        if node.children.is_empty() {
            return;
        }

        // Calculate available area for children
        let x0 = node.x + self.padding;
        let y0 = node.y + self.padding + self.padding_top;
        let x1 = node.x + node.width - self.padding;
        let y1 = node.y + node.rect_height - self.padding;

        if x1 <= x0 || y1 <= y0 {
            return;
        }

        // Apply appropriate tiling method
        match self.tiling {
            TilingMethod::Squarify => self.tile_squarify(node, x0, y0, x1, y1),
            TilingMethod::Binary => self.tile_binary(node, x0, y0, x1, y1),
            TilingMethod::Slice => self.tile_slice(node, x0, y0, x1, y1),
            TilingMethod::Dice => self.tile_dice(node, x0, y0, x1, y1),
            TilingMethod::SliceDice => {
                if node.depth % 2 == 0 {
                    self.tile_slice(node, x0, y0, x1, y1);
                } else {
                    self.tile_dice(node, x0, y0, x1, y1);
                }
            }
        }

        // Recursively tile children
        for child in &mut node.children {
            self.tile_node(child);
        }
    }

    /// Slice tiling (horizontal slices)
    fn tile_slice<T>(&self, node: &mut HierarchyNode<T>, x0: f64, y0: f64, x1: f64, y1: f64) {
        let total = node.value;
        if total <= 0.0 {
            return;
        }

        let height = y1 - y0;
        let mut y = y0;

        for child in &mut node.children {
            let h = (child.value / total) * height;
            child.x = x0;
            child.y = y;
            child.width = x1 - x0;
            child.rect_height = h;
            y += h;
        }
    }

    /// Dice tiling (vertical slices)
    fn tile_dice<T>(&self, node: &mut HierarchyNode<T>, x0: f64, y0: f64, x1: f64, y1: f64) {
        let total = node.value;
        if total <= 0.0 {
            return;
        }

        let width = x1 - x0;
        let mut x = x0;

        for child in &mut node.children {
            let w = (child.value / total) * width;
            child.x = x;
            child.y = y0;
            child.width = w;
            child.rect_height = y1 - y0;
            x += w;
        }
    }

    /// Binary tiling (alternating splits)
    fn tile_binary<T>(&self, node: &mut HierarchyNode<T>, x0: f64, y0: f64, x1: f64, y1: f64) {
        let n = node.children.len();
        if n == 0 {
            return;
        }

        if n == 1 {
            node.children[0].x = x0;
            node.children[0].y = y0;
            node.children[0].width = x1 - x0;
            node.children[0].rect_height = y1 - y0;
            return;
        }

        // Find best split point
        let total = node.value;
        let target = total / 2.0;
        let mut sum = 0.0;
        let mut split_index = 0;

        for (i, child) in node.children.iter().enumerate() {
            sum += child.value;
            if sum >= target {
                split_index = i;
                break;
            }
        }
        split_index = split_index.max(0).min(n - 2);

        let left_sum: f64 = node.children[..=split_index].iter().map(|c| c.value).sum();
        let ratio = if total > 0.0 { left_sum / total } else { 0.5 };

        // Split horizontally or vertically based on aspect ratio
        let width = x1 - x0;
        let height = y1 - y0;

        if width > height {
            // Vertical split
            let split_x = x0 + width * ratio;
            self.tile_binary_range(node, 0, split_index + 1, x0, y0, split_x, y1);
            self.tile_binary_range(node, split_index + 1, n, split_x, y0, x1, y1);
        } else {
            // Horizontal split
            let split_y = y0 + height * ratio;
            self.tile_binary_range(node, 0, split_index + 1, x0, y0, x1, split_y);
            self.tile_binary_range(node, split_index + 1, n, x0, split_y, x1, y1);
        }
    }

    fn tile_binary_range<T>(
        &self,
        node: &mut HierarchyNode<T>,
        start: usize,
        end: usize,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) {
        let count = end - start;
        if count == 0 {
            return;
        }

        if count == 1 {
            node.children[start].x = x0;
            node.children[start].y = y0;
            node.children[start].width = x1 - x0;
            node.children[start].rect_height = y1 - y0;
            return;
        }

        let total: f64 = node.children[start..end].iter().map(|c| c.value).sum();
        let target = total / 2.0;
        let mut sum = 0.0;
        let mut split_index = start;

        for i in start..end {
            sum += node.children[i].value;
            if sum >= target {
                split_index = i;
                break;
            }
        }

        let left_sum: f64 = node.children[start..=split_index].iter().map(|c| c.value).sum();
        let ratio = if total > 0.0 { left_sum / total } else { 0.5 };

        let width = x1 - x0;
        let height = y1 - y0;

        if width > height {
            let split_x = x0 + width * ratio;
            self.tile_binary_range(node, start, split_index + 1, x0, y0, split_x, y1);
            self.tile_binary_range(node, split_index + 1, end, split_x, y0, x1, y1);
        } else {
            let split_y = y0 + height * ratio;
            self.tile_binary_range(node, start, split_index + 1, x0, y0, x1, split_y);
            self.tile_binary_range(node, split_index + 1, end, x0, split_y, x1, y1);
        }
    }

    /// Squarified tiling (produces square-ish rectangles)
    fn tile_squarify<T>(&self, node: &mut HierarchyNode<T>, x0: f64, y0: f64, x1: f64, y1: f64) {
        let total = node.value;
        if total <= 0.0 || node.children.is_empty() {
            return;
        }

        // Sort children by value (descending) for better squarification
        let mut indices: Vec<usize> = (0..node.children.len()).collect();
        indices.sort_by(|&a, &b| {
            node.children[b]
                .value
                .partial_cmp(&node.children[a].value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut remaining_area = (x1 - x0) * (y1 - y0);
        let mut remaining_value = total;

        let mut curr_x = x0;
        let mut curr_y = y0;
        let mut curr_w = x1 - x0;
        let mut curr_h = y1 - y0;

        let mut row_start = 0;
        let mut row_values: Vec<f64> = Vec::new();
        let mut row_sum = 0.0;

        for (pos, &idx) in indices.iter().enumerate() {
            let value = node.children[idx].value;

            if row_values.is_empty() {
                row_values.push(value);
                row_sum = value;
                continue;
            }

            // Calculate aspect ratio with and without this item
            let short_side = curr_w.min(curr_h);
            let row_area = row_sum / remaining_value * remaining_area;
            let row_length = row_area / short_side;

            let worst_current = self.worst_ratio(&row_values, row_length);

            row_values.push(value);
            row_sum += value;
            let new_row_area = row_sum / remaining_value * remaining_area;
            let new_row_length = new_row_area / short_side;
            let worst_with_new = self.worst_ratio(&row_values, new_row_length);

            if worst_with_new > worst_current && row_values.len() > 1 {
                // Adding this item makes it worse, so finalize current row
                row_values.pop();
                row_sum -= value;

                // Layout the row
                let row_area = row_sum / remaining_value * remaining_area;
                let (new_x, new_y, new_w, new_h) = self.layout_row(
                    node,
                    &indices[row_start..pos],
                    &row_values,
                    row_area,
                    curr_x,
                    curr_y,
                    curr_w,
                    curr_h,
                );

                remaining_area -= row_area;
                remaining_value -= row_sum;
                curr_x = new_x;
                curr_y = new_y;
                curr_w = new_w;
                curr_h = new_h;

                // Start new row with this item
                row_start = pos;
                row_values = vec![value];
                row_sum = value;
            }
        }

        // Layout final row
        if !row_values.is_empty() {
            let row_area = remaining_area;
            self.layout_row(
                node,
                &indices[row_start..],
                &row_values,
                row_area,
                curr_x,
                curr_y,
                curr_w,
                curr_h,
            );
        }
    }

    /// Calculate worst aspect ratio in a row
    fn worst_ratio(&self, values: &[f64], length: f64) -> f64 {
        if values.is_empty() || length <= 0.0 {
            return f64::INFINITY;
        }

        let sum: f64 = values.iter().sum();
        let length_sq = length * length;

        let mut worst = 0.0_f64;
        for &v in values {
            let w = v / sum * length;
            let h = v / w;
            let ratio = (w / h).max(h / w);
            worst = worst.max(ratio);
        }
        worst
    }

    /// Layout a row of nodes
    fn layout_row<T>(
        &self,
        node: &mut HierarchyNode<T>,
        indices: &[usize],
        values: &[f64],
        area: f64,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> (f64, f64, f64, f64) {
        let sum: f64 = values.iter().sum();
        if sum <= 0.0 {
            return (x, y, w, h);
        }

        let horizontal = w >= h;
        let length = if horizontal { area / h } else { area / w };

        let mut pos = if horizontal { x } else { y };

        for (i, &idx) in indices.iter().enumerate() {
            let v = values[i];
            let size = if sum > 0.0 { v / sum * (if horizontal { h } else { w }) } else { 0.0 };

            if horizontal {
                node.children[idx].x = pos;
                node.children[idx].y = y;
                node.children[idx].width = length;
                node.children[idx].rect_height = size;
                pos += size;
            } else {
                node.children[idx].x = x;
                node.children[idx].y = pos;
                node.children[idx].width = size;
                node.children[idx].rect_height = length;
                pos += size;
            }
        }

        // Return remaining area
        if horizontal {
            (x + length, y, w - length, h)
        } else {
            (x, y + length, w, h - length)
        }
    }

    /// Round coordinates to whole pixels
    fn round_coords<T>(&self, node: &mut HierarchyNode<T>) {
        node.x = node.x.round();
        node.y = node.y.round();
        node.width = node.width.round();
        node.rect_height = node.rect_height.round();

        for child in &mut node.children {
            self.round_coords(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tree() -> HierarchyNode<String> {
        let mut root = HierarchyNode::from_label("root", 0.0);
        root.add_child(HierarchyNode::from_label("A", 30.0));
        root.add_child(HierarchyNode::from_label("B", 20.0));
        root.add_child(HierarchyNode::from_label("C", 50.0));
        root
    }

    #[test]
    fn test_treemap_layout_new() {
        let layout = TreemapLayout::new();
        assert_eq!(layout.width, 1.0);
        assert_eq!(layout.height, 1.0);
    }

    #[test]
    fn test_treemap_layout_size() {
        let layout = TreemapLayout::new().size(100.0, 100.0);
        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
    }

    #[test]
    fn test_treemap_layout_slice() {
        let tree = make_tree();
        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .tiling(TilingMethod::Slice);

        let positioned = layout.layout(&tree);

        // Root should cover the entire area
        assert_eq!(positioned.x, 0.0);
        assert_eq!(positioned.y, 0.0);
        assert_eq!(positioned.width, 100.0);
        assert_eq!(positioned.rect_height, 100.0);

        // Children should be sliced horizontally
        let total: f64 = positioned.children.iter().map(|c| c.value).sum();
        for child in &positioned.children {
            assert_eq!(child.width, 100.0);
            let expected_height = child.value / total * 100.0;
            assert!((child.rect_height - expected_height).abs() < 0.01);
        }
    }

    #[test]
    fn test_treemap_layout_dice() {
        let tree = make_tree();
        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .tiling(TilingMethod::Dice);

        let positioned = layout.layout(&tree);

        // Children should be sliced vertically
        for child in &positioned.children {
            assert_eq!(child.rect_height, 100.0);
        }
    }

    #[test]
    fn test_treemap_layout_area_preservation() {
        let tree = make_tree();
        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .tiling(TilingMethod::Squarify);

        let positioned = layout.layout(&tree);

        // Total area of children should equal parent area (minus padding)
        let total_area = 100.0 * 100.0;
        let children_area: f64 = positioned
            .children
            .iter()
            .map(|c| c.width * c.rect_height)
            .sum();

        assert!((children_area - total_area).abs() < 1.0);
    }

    #[test]
    fn test_treemap_layout_with_padding() {
        let tree = make_tree();
        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .padding(5.0)
            .tiling(TilingMethod::Slice);

        let positioned = layout.layout(&tree);

        // Children should start after padding
        for child in &positioned.children {
            assert!(child.x >= 5.0);
        }
    }

    #[test]
    fn test_treemap_layout_round() {
        let tree = make_tree();
        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .round(true)
            .tiling(TilingMethod::Slice);

        let positioned = layout.layout(&tree);

        for child in &positioned.children {
            assert_eq!(child.x, child.x.round());
            assert_eq!(child.y, child.y.round());
        }
    }

    #[test]
    fn test_treemap_nested() {
        let mut root = HierarchyNode::from_label("root", 0.0);

        let mut child1 = HierarchyNode::from_label("child1", 0.0);
        child1.add_child(HierarchyNode::from_label("leaf1", 25.0));
        child1.add_child(HierarchyNode::from_label("leaf2", 25.0));

        let mut child2 = HierarchyNode::from_label("child2", 0.0);
        child2.add_child(HierarchyNode::from_label("leaf3", 50.0));

        root.add_child(child1);
        root.add_child(child2);

        let layout = TreemapLayout::new()
            .size(100.0, 100.0)
            .tiling(TilingMethod::Slice);

        let positioned = layout.layout(&root);

        // After sum(), child1 should have value 50, child2 should have value 50
        assert_eq!(positioned.children[0].value, 50.0);
        assert_eq!(positioned.children[1].value, 50.0);

        // Leaves should be positioned within their parent
        let leaf1 = &positioned.children[0].children[0];
        let parent = &positioned.children[0];
        assert!(leaf1.x >= parent.x);
        assert!(leaf1.y >= parent.y);
    }
}
