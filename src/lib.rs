use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

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
pub fn transform(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut RemoveDataTestId);
    program
}
