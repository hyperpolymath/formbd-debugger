// SPDX-License-Identifier: AGPL-3.0-or-later
//! Constraint tree visualization widget

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

/// A constraint with its status
pub struct ConstraintNode {
    pub name: String,
    pub constraint_type: String,
    pub satisfied: bool,
    pub violation_message: Option<String>,
}

/// Constraint tree widget
pub struct ConstraintTreeWidget<'a> {
    constraints: &'a [ConstraintNode],
    block: Option<Block<'a>>,
}

impl<'a> ConstraintTreeWidget<'a> {
    pub fn new(constraints: &'a [ConstraintNode]) -> Self {
        Self {
            constraints,
            block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for ConstraintTreeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self
            .block
            .unwrap_or_else(|| Block::default().borders(Borders::ALL).title("Constraints"));
        let inner = block.inner(area);
        block.render(area, buf);

        for (i, constraint) in self.constraints.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }

            let (icon, style) = if constraint.satisfied {
                ("✓", Style::default().fg(Color::Green))
            } else {
                ("✗", Style::default().fg(Color::Red))
            };

            let line = format!(
                "{} {} ({})",
                icon, constraint.name, constraint.constraint_type
            );

            buf.set_string(inner.x, inner.y + i as u16, &line, style);
        }
    }
}
