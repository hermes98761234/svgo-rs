use crate::ast::{Document, Element, Node};

/// Controls how the visitor proceeds after visiting a node.
#[derive(Debug, Clone, PartialEq)]
pub enum VisitAction {
    /// Continue normally — visit children, then fire exit hook.
    Continue,
    /// Skip visiting children of this node, but still fire exit hook.
    SkipChildren,
    /// Remove this node from its parent. Exit hook still fires.
    Remove,
}

/// Context passed to visitor hooks, carrying ancestor element names.
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// Ancestor element names, outermost first. Empty for the root document.
    pub ancestors: Vec<String>,
}

/// A visitor that walks the AST depth-first with mutation support.
///
/// Each enter-style hook returns a `VisitAction` controlling traversal.
/// Default implementation returns `Continue`.
pub trait Visitor {
    fn element_enter(&mut self, el: &mut Element, ctx: &Context) -> VisitAction {
        let _ = (el, ctx);
        VisitAction::Continue
    }
    fn element_exit(&mut self, el: &mut Element, ctx: &Context) {
        let _ = (el, ctx);
    }
    fn text(&mut self, text: &mut String, ctx: &Context) -> VisitAction {
        let _ = (text, ctx);
        VisitAction::Continue
    }
    fn comment(&mut self, c: &mut String, ctx: &Context) -> VisitAction {
        let _ = (c, ctx);
        VisitAction::Continue
    }
    fn doctype(&mut self, d: &mut String, ctx: &Context) -> VisitAction {
        let _ = (d, ctx);
        VisitAction::Continue
    }
    fn instruction(&mut self, name: &mut String, value: &mut String, ctx: &Context) -> VisitAction {
        let _ = (name, value, ctx);
        VisitAction::Continue
    }
}

/// Visit a document depth-first, calling visitor hooks and supporting removal.
pub fn visit(doc: &mut Document, v: &mut dyn Visitor) {
    let mut ctx = Context::default();
    visit_children(&mut doc.children, v, &mut ctx);
}

/// Visit a mutable slice of nodes, removing any that returned `Remove`.
fn visit_children(children: &mut Vec<Node>, v: &mut dyn Visitor, ctx: &mut Context) {
    let mut i = 0;
    while i < children.len() {
        match &mut children[i] {
            Node::Element(el) => {
                let enter_action = v.element_enter(el, ctx);
                match enter_action {
                    VisitAction::Remove => {
                        v.element_exit(el, ctx);
                        children.remove(i);
                        continue;
                    }
                    VisitAction::SkipChildren => {
                        v.element_exit(el, ctx);
                        i += 1;
                        continue;
                    }
                    VisitAction::Continue => {
                        ctx.ancestors.push(el.name.clone());
                        visit_children(&mut el.children, v, ctx);
                        ctx.ancestors.pop();
                        v.element_exit(el, ctx);
                        i += 1;
                        continue;
                    }
                }
            }
            Node::Text(text) => {
                let action = v.text(text, ctx);
                if action == VisitAction::Remove {
                    children.remove(i);
                    continue;
                }
                i += 1;
                continue;
            }
            Node::Comment(c) => {
                let action = v.comment(c, ctx);
                if action == VisitAction::Remove {
                    children.remove(i);
                    continue;
                }
                i += 1;
                continue;
            }
            Node::Doctype(d) => {
                let action = v.doctype(d, ctx);
                if action == VisitAction::Remove {
                    children.remove(i);
                    continue;
                }
                i += 1;
                continue;
            }
            Node::Instruction { name, value } => {
                let action = v.instruction(name, value, ctx);
                if action == VisitAction::Remove {
                    children.remove(i);
                    continue;
                }
                i += 1;
                continue;
            }
            Node::CData(_) => {
                i += 1;
                continue;
            }
        }
    }
}
