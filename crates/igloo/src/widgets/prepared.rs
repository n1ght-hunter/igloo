//! Owns the renderer data borrowed by a plugin's current view.

use std::collections::HashMap;

use iced::widget::{combo_box, markdown};

use super::{Message, WrapperRenderer, WrapperTheme, build_element};
use crate::bindings::iced::app::widgets::{Node, NodeId, ViewTree};

#[derive(Debug)]
struct Markdown {
    content: String,
    items: Vec<markdown::Item>,
}

/// Retains unchanged widget state and releases data absent from the next tree.
#[derive(Debug)]
pub struct PreparedView {
    tree: ViewTree,
    combos: HashMap<NodeId, combo_box::State<String>>,
    markdown: HashMap<NodeId, Markdown>,
}

impl PreparedView {
    pub fn new(tree: ViewTree) -> Self {
        let mut prepared = Self {
            tree: ViewTree {
                root: 0,
                nodes: Vec::new(),
            },
            combos: HashMap::new(),
            markdown: HashMap::new(),
        };
        prepared.refresh(tree);
        prepared
    }

    /// Reuses state only when both the node ID and its source data match.
    pub fn refresh(&mut self, tree: ViewTree) {
        let mut combos = std::mem::take(&mut self.combos);
        let mut markdown = std::mem::take(&mut self.markdown);
        for (id, node) in tree.nodes.iter().enumerate() {
            let id = id as NodeId;
            match node {
                Node::ComboBox(node) => {
                    let state = combos
                        .remove(&id)
                        .filter(|state| state.options() == node.options)
                        .unwrap_or_else(|| combo_box::State::new(node.options.clone()));
                    self.combos.insert(id, state);
                }
                Node::Markdown(node) => {
                    let state = markdown
                        .remove(&id)
                        .filter(|state| state.content == node.content)
                        .unwrap_or_else(|| Markdown {
                            content: node.content.clone(),
                            items: markdown::parse(&node.content).collect(),
                        });
                    self.markdown.insert(id, state);
                }
                _ => {}
            }
        }
        self.tree = tree;
    }

    pub fn render<'a, Theme, Renderer>(&'a self) -> impl iced::Widget<Message, Theme, Renderer> + 'a
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'static,
    {
        let mut nodes = self.tree.nodes.iter().cloned().map(Some).collect();
        build_element(&mut nodes, self.tree.root, self)
    }

    pub(super) fn combo(&self, id: NodeId) -> &combo_box::State<String> {
        &self.combos[&id]
    }

    pub(super) fn markdown(&self, id: NodeId) -> &[markdown::Item] {
        &self.markdown[&id].items
    }

    #[cfg(test)]
    pub(crate) fn tree(&self) -> &ViewTree {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::iced::app::widgets::{ComboBoxNode, MarkdownNode};

    fn tree(options: &[&str], content: &str) -> ViewTree {
        ViewTree {
            root: 0,
            nodes: vec![
                Node::ComboBox(ComboBoxNode {
                    options: options.iter().map(|option| (*option).to_owned()).collect(),
                    placeholder: String::new(),
                    selected: None,
                    on_selected: 1,
                    on_input: None,
                    on_option_hovered: None,
                    on_open: None,
                    on_close: None,
                    padding: None,
                    size: None,
                    line_height: None,
                    width: None,
                }),
                Node::Markdown(MarkdownNode {
                    content: content.into(),
                    on_link_click: 2,
                }),
            ],
        }
    }

    #[test]
    fn unchanged_content_reuses_allocations_even_when_callbacks_change() {
        let mut source = tree(&["one", "two"], "# Heading");
        let mut prepared = PreparedView::new(source.clone());
        let options = prepared.combos[&0].options().as_ptr();
        let items = prepared.markdown[&1].items.as_ptr();
        for callback in 3..100 {
            let Node::ComboBox(combo) = &mut source.nodes[0] else {
                unreachable!()
            };
            combo.on_selected = callback;
            let Node::Markdown(markdown) = &mut source.nodes[1] else {
                unreachable!()
            };
            markdown.on_link_click = callback;
            prepared.refresh(source.clone());
            assert_eq!(options, prepared.combos[&0].options().as_ptr());
            assert_eq!(items, prepared.markdown[&1].items.as_ptr());
        }
        assert_eq!(prepared.combos.len(), 1);
        assert_eq!(prepared.markdown.len(), 1);
    }

    #[test]
    fn changed_content_is_rebuilt_and_removed_nodes_are_released() {
        let mut prepared = PreparedView::new(tree(&["before"], "before"));
        prepared.refresh(tree(&["after"], "# After"));
        assert_eq!(prepared.combos[&0].options(), &["after"]);
        assert_eq!(prepared.markdown[&1].content, "# After");
        let expected = markdown::parse("# After").collect::<Vec<_>>();
        assert_eq!(
            format!("{:?}", prepared.markdown[&1].items),
            format!("{expected:?}")
        );
        prepared.refresh(ViewTree {
            root: 0,
            nodes: Vec::new(),
        });
        assert!(prepared.combos.is_empty());
        assert!(prepared.markdown.is_empty());
    }

    #[test]
    fn changing_widget_kind_at_the_same_id_discards_old_state() {
        let mut prepared = PreparedView::new(tree(&["one"], "text"));
        let mut changed = tree(&["one"], "text");
        changed.nodes.swap(0, 1);
        prepared.refresh(changed);
        assert_eq!(prepared.combos.keys().copied().collect::<Vec<_>>(), [1]);
        assert_eq!(prepared.markdown.keys().copied().collect::<Vec<_>>(), [0]);
    }
}
