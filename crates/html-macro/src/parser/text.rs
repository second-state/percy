use crate::parser::HtmlParser;
use crate::tag::Tag;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::cmp::max;

impl HtmlParser {
    /// Parse an incoming Tag::Text text node
    pub(crate) fn parse_text(
        &mut self,
        text: &str,
        text_start: Span,
        text_end: Span,
        next_tag: Option<&Tag>,
    ) {
        let mut text = text.to_string();

        if self.should_insert_space_before_text(&text_start) {
            text = " ".to_string() + &text;
        }

        let should_insert_space_after_text = match next_tag {
            Some(Tag::Close {
                first_angle_bracket_span,
                ..
            }) => self.should_insert_space_after_text(&text_end, first_angle_bracket_span, true),
            Some(Tag::Braced { brace_span, .. }) => {
                self.should_insert_space_after_text(&text_end, brace_span, false)
            }
            _ => false,
        };
        if should_insert_space_after_text {
            text += " ";
        }

        let idx = &mut self.current_node_idx;
        let parent_to_children = &mut self.parent_to_children;
        let parent_stack = &mut self.parent_stack;
        let tokens = &mut self.tokens;
        let node_order = &mut self.node_order;

        if *idx == 0 {
            node_order.push(0);
            // TODO: This is just a consequence of bad code. We're pushing this to make
            // things work but in reality a text node isn't a parent ever.
            // Just need to make the code DRY / refactor so that we can make things make
            // sense vs. just bolting things together.
            parent_stack.push((0, Ident::new("unused", Span::call_site())));
        }

        let var_name = Ident::new(format!("node_{}", idx).as_str(), Span::call_site());

        let text_node = quote! {
            let mut #var_name = VirtualNode::text(#text);
        };

        tokens.push(text_node);

        if *idx == 0 {
            *idx += 1;
            return;
        }

        let parent_idx = &parent_stack[parent_stack.len() - 1];

        node_order.push(*idx);

        parent_to_children
            .get_mut(&parent_idx.0)
            .expect("Parent of this text node")
            .push(*idx);

        *idx += 1;
    }

    fn should_insert_space_before_text(&self, start_span: &Span) -> bool {
        // If the first thing that we encounter in our HTML macro is test we don't
        // need to insert any space before it.
        if self
            .recent_span_locations
            .most_recent_open_tag_end
            .is_none()
            && self.recent_span_locations.most_recent_block_start.is_none()
        {
            return false;
        }

        let most_recent_open_tag_end = self
            .recent_span_locations
            .most_recent_open_tag_end
            .as_ref()
            .unwrap();

        return start_span.start().line != most_recent_open_tag_end.line
            || start_span.start().column - most_recent_open_tag_end.column > 1;
    }

    fn should_insert_space_after_text(
        &self,
        text_end: &Span,
        next_span: &Span,
        adjust_span_rustc_bug: bool,
    ) -> bool {
        if text_end.end().line != next_span.start().line {
            return true;
        }

        if adjust_span_rustc_bug {
            // TODO: The angle bracket has an incorrect span that has a column that is
            // one less than what it's supposed to be so we use the token after the span
            // instead. We can use the angle bracket again after rust-lang/rust #58958
            // https://github.com/rust-lang/rust/issues/58958
            // Once the issue closes we can delete this if and keep the else clause
            return next_span.start().column - text_end.end().column > 0;
        } else {
            return next_span.start().column - text_end.end().column > 1;
        }
    }
}
