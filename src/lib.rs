use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use swc_plugin_macro::plugin_transform;

struct RemoveDataTestId;

impl VisitMut for RemoveDataTestId {
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        n.visit_mut_children_with(self);
        n.attrs.retain(|attr| {
            match attr {
                JSXAttrOrSpread::JSXAttr(a) => {
                    if let JSXAttrName::Ident(ident) = &a.name {
                        return ident.sym != *"data-testid";
                    }
                    true
                }
                _ => true,
            }
        });
    }
}

#[plugin_transform]
pub fn transform(program: Program) -> Program {
    let mut plugin = RemoveDataTestId;
    let mut program = program;
    program.visit_mut_with(&mut plugin);
    program
}
