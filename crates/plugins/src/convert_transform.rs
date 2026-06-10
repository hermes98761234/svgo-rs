use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::transforms::{
    js2transform, matrix_to_transform, round_transform, transform2js, transforms_multiply,
    TransformItem, TransformParams,
};

/// Collapses multiple transformations and optimizes them.
pub struct ConvertTransform;

impl Plugin for ConvertTransform {
    fn name(&self) -> &'static str {
        "convertTransform"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let p = TransformParams {
            convert_to_shorts: params
                .get("convertToShorts")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            deg_precision: params
                .get("degPrecision")
                .and_then(|v| v.as_u64())
                .map(|v| v as u8),
            float_precision: params
                .get("floatPrecision")
                .and_then(|v| v.as_u64())
                .map(|v| v as u8)
                .unwrap_or(3),
            transform_precision: params
                .get("transformPrecision")
                .and_then(|v| v.as_u64())
                .map(|v| v as u8)
                .unwrap_or(5),
            matrix_to_transform: params
                .get("matrixToTransform")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            short_translate: params
                .get("shortTranslate")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            short_scale: params
                .get("shortScale")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            short_rotate: params
                .get("shortRotate")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            remove_useless: params
                .get("removeUseless")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            collapse_into_one: params
                .get("collapseIntoOne")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            leading_zero: params
                .get("leadingZero")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            negative_extra_space: params
                .get("negativeExtraSpace")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        };

        struct V {
            params: TransformParams,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                for attr_name in &["transform", "gradientTransform", "patternTransform"] {
                    if let Some(value) = el.attributes.get(*attr_name) {
                        let value = value.clone();
                        convert_transform_attr(el, attr_name, &self.params, value);
                    }
                }
                VisitAction::Continue
            }
        }
        let mut v = V { params: p };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

fn convert_transform_attr(
    el: &mut svgo_core::ast::Element,
    attr_name: &str,
    base_params: &TransformParams,
    value: String,
) {
    let mut data = transform2js(&value);
    let params = define_precision(&data, base_params);

    if params.collapse_into_one && data.len() > 1 {
        data = vec![transforms_multiply(&data)];
    }

    if params.convert_to_shorts {
        data = convert_to_shorts(data, &params);
    } else {
        for item in &mut data {
            round_transform(item, &params);
        }
    }

    if params.remove_useless {
        data = remove_useless(&data);
    }

    if data.is_empty() {
        el.attributes.shift_remove(attr_name);
    } else {
        el.attributes
            .insert(attr_name.to_string(), js2transform(&data, &params));
    }
}

fn define_precision(data: &[TransformItem], base_params: &TransformParams) -> TransformParams {
    let mut params = base_params.clone();
    let mut matrix_data = Vec::new();
    for item in data {
        if item.name == "matrix" {
            matrix_data.extend_from_slice(&item.data[..4.min(item.data.len())]);
        }
    }
    if !matrix_data.is_empty() {
        let max_float_digits = matrix_data
            .iter()
            .map(|n| {
                let s = n.to_string();
                let dot_pos = s.find('.').unwrap_or(s.len());
                s.len() - dot_pos
            })
            .max()
            .unwrap_or(0);
        params.transform_precision = params
            .transform_precision
            .min(max_float_digits as u8)
            .min(params.transform_precision);
    }
    if params.deg_precision.is_none() {
        params.deg_precision = Some(0.max(params.float_precision as i32 - 2) as u8);
    }
    params
}

fn convert_to_shorts(
    mut transforms: Vec<TransformItem>,
    params: &TransformParams,
) -> Vec<TransformItem> {
    let mut i = 0;
    while i < transforms.len() {
        // convert matrix to short aliases
        if params.matrix_to_transform && transforms[i].name == "matrix" {
            let decomposed = matrix_to_transform(&transforms[i], params);
            let orig_len = js2transform(&[transforms[i].clone()], params).len();
            let new_len = js2transform(&decomposed, params).len();
            if new_len <= orig_len {
                transforms.splice(i..=i, decomposed);
            }
        }

        if i >= transforms.len() {
            break;
        }
        round_transform(&mut transforms[i], params);

        // short translate: translate(10 0) -> translate(10)
        if params.short_translate
            && transforms[i].name == "translate"
            && transforms[i].data.len() == 2
            && transforms[i].data[1] == 0.0
        {
            transforms[i].data.pop();
        }

        // short scale: scale(2 2) -> scale(2)
        if params.short_scale
            && transforms[i].name == "scale"
            && transforms[i].data.len() == 2
            && transforms[i].data[0] == transforms[i].data[1]
        {
            transforms[i].data.pop();
        }

        // short rotate: translate(cx cy) rotate(a) translate(-cx -cy) -> rotate(a cx cy)
        if params.short_rotate
            && i >= 2
            && transforms.get(i.wrapping_sub(2)).map(|t| t.name.as_str()) == Some("translate")
            && transforms.get(i.wrapping_sub(1)).map(|t| t.name.as_str()) == Some("rotate")
            && transforms[i].name == "translate"
        {
            let prev_translate = &transforms[i - 2];
            if prev_translate.data.len() >= 2
                && transforms[i].data.len() >= 2
                && (prev_translate.data[0] + transforms[i].data[0]).abs() < 1e-10
                && (prev_translate.data[1] + transforms[i].data[1]).abs() < 1e-10
            {
                let rotate_data = transforms[i - 1].data[0];
                transforms.splice(
                    i - 2..=i,
                    [TransformItem {
                        name: "rotate".to_string(),
                        data: vec![rotate_data, prev_translate.data[0], prev_translate.data[1]],
                    }],
                );
                i = i.saturating_sub(2);
                continue;
            }
        }

        i += 1;
    }
    transforms
}

fn remove_useless(transforms: &[TransformItem]) -> Vec<TransformItem> {
    transforms
        .iter()
        .filter(|t| match t.name.as_str() {
            "translate" | "rotate" | "skewX" | "skewY" => {
                if t.data.is_empty() {
                    return true;
                }
                if t.name == "translate" {
                    return !(t.data[0] == 0.0 && t.data.get(1).copied().unwrap_or(0.0) == 0.0);
                }
                t.data[0] != 0.0
            }
            "scale" => !(t.data[0] == 1.0 && t.data.get(1).copied().unwrap_or(1.0) == 1.0),
            "matrix" => {
                !(t.data[0] == 1.0
                    && t.data[3] == 1.0
                    && t.data[1] == 0.0
                    && t.data[2] == 0.0
                    && t.data[4] == 0.0
                    && t.data[5] == 0.0)
            }
            _ => true,
        })
        .cloned()
        .collect()
}
