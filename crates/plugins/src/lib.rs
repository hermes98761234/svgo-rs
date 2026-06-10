pub mod collections;

pub mod cleanup_enable_background;
pub mod remove_comments;
pub mod remove_desc;
pub mod remove_doctype;
pub mod remove_editors_ns_data;
pub mod remove_empty_attrs;
pub mod remove_empty_containers;
pub mod remove_empty_text;
pub mod remove_hidden_elems;
pub mod remove_metadata;
pub mod remove_non_inheritable_group_attrs;
pub mod remove_unused_ns;
pub mod remove_useless_defs;
pub mod remove_view_box;
pub mod remove_xml_proc_inst;

use svgo_core::plugin::Registry;

/// Register all batch A plugins into the given registry.
pub fn register_all(r: &mut Registry) {
    use cleanup_enable_background::CleanupEnableBackground;
    use remove_comments::RemoveComments;
    use remove_desc::RemoveDesc;
    use remove_doctype::RemoveDoctype;
    use remove_editors_ns_data::RemoveEditorsNSData;
    use remove_empty_attrs::RemoveEmptyAttrs;
    use remove_empty_containers::RemoveEmptyContainers;
    use remove_empty_text::RemoveEmptyText;
    use remove_hidden_elems::RemoveHiddenElems;
    use remove_metadata::RemoveMetadata;
    use remove_non_inheritable_group_attrs::RemoveNonInheritableGroupAttrs;
    use remove_unused_ns::RemoveUnusedNS;
    use remove_useless_defs::RemoveUselessDefs;
    use remove_view_box::RemoveViewBox;
    use remove_xml_proc_inst::RemoveXmlProcInst;

    r.register(
        "removeDoctype",
        std::sync::Arc::new(|_| Box::new(RemoveDoctype)),
    );
    r.register(
        "removeXMLProcInst",
        std::sync::Arc::new(|_| Box::new(RemoveXmlProcInst)),
    );
    r.register(
        "removeComments",
        std::sync::Arc::new(|_| Box::new(RemoveComments)),
    );
    r.register(
        "removeMetadata",
        std::sync::Arc::new(|_| Box::new(RemoveMetadata)),
    );
    r.register(
        "removeEditorsNSData",
        std::sync::Arc::new(|_| Box::new(RemoveEditorsNSData)),
    );
    r.register("removeDesc", std::sync::Arc::new(|_| Box::new(RemoveDesc)));
    r.register(
        "removeUselessDefs",
        std::sync::Arc::new(|_| Box::new(RemoveUselessDefs)),
    );
    r.register(
        "removeEmptyAttrs",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyAttrs)),
    );
    r.register(
        "removeEmptyText",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyText)),
    );
    r.register(
        "removeEmptyContainers",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyContainers)),
    );
    r.register(
        "removeHiddenElems",
        std::sync::Arc::new(|_| Box::new(RemoveHiddenElems)),
    );
    r.register(
        "removeUnusedNS",
        std::sync::Arc::new(|_| Box::new(RemoveUnusedNS)),
    );
    r.register(
        "removeViewBox",
        std::sync::Arc::new(|_| Box::new(RemoveViewBox)),
    );
    r.register(
        "cleanupEnableBackground",
        std::sync::Arc::new(|_| Box::new(CleanupEnableBackground)),
    );
    r.register(
        "removeNonInheritableGroupAttrs",
        std::sync::Arc::new(|_| Box::new(RemoveNonInheritableGroupAttrs)),
    );
}
