//! Transform parsing, multiplication, and decomposition.
//!
//! Ported from SVGO's plugins/_transforms.js.

use std::f64::consts::PI;

/// A named transform with its data.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformItem {
    /// Transform name: "matrix", "translate", "scale", "rotate", "skewX", "skewY".
    pub name: String,
    /// Numeric parameters.
    pub data: Vec<f64>,
}

/// Parameters controlling transform rounding and optimization.
#[derive(Debug, Clone)]
pub struct TransformParams {
    pub convert_to_shorts: bool,
    pub deg_precision: Option<u8>,
    pub float_precision: u8,
    pub transform_precision: u8,
    pub matrix_to_transform: bool,
    pub short_translate: bool,
    pub short_scale: bool,
    pub short_rotate: bool,
    pub remove_useless: bool,
    pub collapse_into_one: bool,
    pub leading_zero: bool,
    pub negative_extra_space: bool,
}

impl Default for TransformParams {
    fn default() -> Self {
        Self {
            convert_to_shorts: true,
            deg_precision: None,
            float_precision: 3,
            transform_precision: 5,
            matrix_to_transform: true,
            short_translate: true,
            short_scale: true,
            short_rotate: true,
            remove_useless: true,
            collapse_into_one: true,
            leading_zero: true,
            negative_extra_space: false,
        }
    }
}

/// Regex-like split pattern for transform functions.
const REG_TRANSFORM_SPLIT: &str =
    r"\s*(matrix|translate|scale|rotate|skewX|skewY)\s*\(\s*(.+?)\s*\)";

/// Convert a transform string to a list of TransformItems.
pub fn transform2js(transform_string: &str) -> Vec<TransformItem> {
    let mut transforms: Vec<TransformItem> = Vec::new();
    let mut current_transform: Option<TransformItem> = None;

    // Split on transform function names
    let re = regex::Regex::new(REG_TRANSFORM_SPLIT).unwrap();
    let mut matched_any = false;

    for cap in re.captures_iter(transform_string) {
        matched_any = true;
        let name = cap.get(1).unwrap().as_str().to_string();
        let data_str = cap.get(2).unwrap().as_str();

        let mut data = Vec::new();
        let num_re = regex::Regex::new(r"[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?").unwrap();
        for num_cap in num_re.find_iter(data_str) {
            if let Ok(n) = num_cap.as_str().parse::<f64>() {
                data.push(n);
            }
        }

        transforms.push(TransformItem { name, data });
    }

    if !matched_any {
        return Vec::new();
    }

    // Check if last transform has data
    if let Some(last) = transforms.last() {
        if last.data.is_empty() {
            return Vec::new();
        }
    }

    transforms
}

/// Convert a transform to its 6-element matrix representation [a, b, c, d, e, f].
fn transform_to_matrix(transform: &TransformItem) -> Vec<f64> {
    match transform.name.as_str() {
        "translate" => {
            let tx = transform.data[0];
            let ty = transform.data.get(1).copied().unwrap_or(0.0);
            vec![1.0, 0.0, 0.0, 1.0, tx, ty]
        }
        "scale" => {
            let sx = transform.data[0];
            let sy = transform.data.get(1).copied().unwrap_or(sx);
            vec![sx, 0.0, 0.0, sy, 0.0, 0.0]
        }
        "rotate" => {
            let angle = transform.data[0];
            let rad = angle * PI / 180.0;
            let cos = rad.cos();
            let sin = rad.sin();
            let cx = transform.data.get(1).copied().unwrap_or(0.0);
            let cy = transform.data.get(2).copied().unwrap_or(0.0);
            vec![
                cos,
                sin,
                -sin,
                cos,
                (1.0 - cos) * cx + sin * cy,
                (1.0 - cos) * cy - sin * cx,
            ]
        }
        "skewX" => {
            let angle = transform.data[0];
            let rad = angle * PI / 180.0;
            vec![1.0, 0.0, rad.tan(), 1.0, 0.0, 0.0]
        }
        "skewY" => {
            let angle = transform.data[0];
            let rad = angle * PI / 180.0;
            vec![1.0, rad.tan(), 0.0, 1.0, 0.0, 0.0]
        }
        "matrix" => transform.data.clone(),
        _ => panic!("Unknown transform {}", transform.name),
    }
}

/// Multiply two 6-element transformation matrices.
fn multiply_transform_matrices(a: &[f64], b: &[f64]) -> Vec<f64> {
    vec![
        a[0] * b[0] + a[2] * b[1],
        a[1] * b[0] + a[3] * b[1],
        a[0] * b[2] + a[2] * b[3],
        a[1] * b[2] + a[3] * b[3],
        a[0] * b[4] + a[2] * b[5] + a[4],
        a[1] * b[4] + a[3] * b[5] + a[5],
    ]
}

/// Multiply all transforms into a single matrix transform.
pub fn transforms_multiply(transforms: &[TransformItem]) -> TransformItem {
    let matrix_data: Vec<Vec<f64>> = transforms
        .iter()
        .map(|t| {
            if t.name == "matrix" {
                t.data.clone()
            } else {
                transform_to_matrix(t)
            }
        })
        .collect();

    let result = if matrix_data.is_empty() {
        vec![]
    } else {
        matrix_data
            .into_iter()
            .reduce(|a, b| multiply_transform_matrices(&a, &b))
            .unwrap_or_default()
    };

    TransformItem {
        name: "matrix".to_string(),
        data: result,
    }
}

/// Decompose a matrix into simple transforms (QRAB decomposition).
fn decompose_qrab(matrix: &TransformItem) -> Option<Vec<TransformItem>> {
    let data = &matrix.data;
    let [a, b, c, d, e, f] = [data[0], data[1], data[2], data[3], data[4], data[5]];
    let delta = a * d - b * c;
    if delta == 0.0 {
        return None;
    }
    let r = (a * a + b * b).sqrt();
    if r == 0.0 {
        return None;
    }

    let mut decomposition = Vec::new();
    let cos_of_rotation = a / r;

    if e != 0.0 || f != 0.0 {
        decomposition.push(TransformItem {
            name: "translate".to_string(),
            data: vec![e, f],
        });
    }

    if cos_of_rotation != 1.0 {
        let rotation_angle_rads = cos_of_rotation.acos();
        let angle_deg = if b < 0.0 {
            -rotation_angle_rads * 180.0 / PI
        } else {
            rotation_angle_rads * 180.0 / PI
        };
        decomposition.push(TransformItem {
            name: "rotate".to_string(),
            data: vec![angle_deg, 0.0, 0.0],
        });
    }

    let sx = r;
    let sy = delta / r;
    if sx != 1.0 || sy != 1.0 {
        decomposition.push(TransformItem {
            name: "scale".to_string(),
            data: vec![sx, sy],
        });
    }

    let ac_plus_bd = a * c + b * d;
    if ac_plus_bd != 0.0 {
        let skew_angle = (ac_plus_bd / (a * a + b * b)).atan() * 180.0 / PI;
        decomposition.push(TransformItem {
            name: "skewX".to_string(),
            data: vec![skew_angle],
        });
    }

    Some(decomposition)
}

/// Decompose a matrix into simple transforms (QRCD decomposition).
fn decompose_qrcd(matrix: &TransformItem) -> Option<Vec<TransformItem>> {
    let data = &matrix.data;
    let [a, b, c, d, e, f] = [data[0], data[1], data[2], data[3], data[4], data[5]];
    let delta = a * d - b * c;
    if delta == 0.0 {
        return None;
    }
    let s = (c * c + d * d).sqrt();
    if s == 0.0 {
        return None;
    }

    let mut decomposition = Vec::new();

    if e != 0.0 || f != 0.0 {
        decomposition.push(TransformItem {
            name: "translate".to_string(),
            data: vec![e, f],
        });
    }

    let rotation_angle_rads = PI / 2.0 - (if d < 0.0 { -1.0 } else { 1.0 }) * (-c / s).acos();
    decomposition.push(TransformItem {
        name: "rotate".to_string(),
        data: vec![rotation_angle_rads * 180.0 / PI, 0.0, 0.0],
    });

    let sx = delta / s;
    let sy = s;
    if sx != 1.0 || sy != 1.0 {
        decomposition.push(TransformItem {
            name: "scale".to_string(),
            data: vec![sx, sy],
        });
    }

    let ac_plus_bd = a * c + b * d;
    if ac_plus_bd != 0.0 {
        let skew_angle = (ac_plus_bd / (c * c + d * d)).atan() * 180.0 / PI;
        decomposition.push(TransformItem {
            name: "skewY".to_string(),
            data: vec![skew_angle],
        });
    }

    Some(decomposition)
}

/// Get all possible decompositions of a matrix.
fn get_decompositions(matrix: &TransformItem) -> Vec<Vec<TransformItem>> {
    let mut decompositions = Vec::new();
    if let Some(d) = decompose_qrab(matrix) {
        decompositions.push(d);
    }
    if let Some(d) = decompose_qrcd(matrix) {
        decompositions.push(d);
    }
    decompositions
}

/// Round a number to the given number of decimal places.
fn to_fixed(num: f64, precision: u8) -> f64 {
    let pow = 10_f64.powi(precision as i32);
    (num * pow).round() / pow
}

/// Remove leading zero from float string.
fn remove_leading_zero(value: f64) -> String {
    let s = format!("{value}");
    if value > 0.0 && value < 1.0 {
        if let Some(rest) = s.strip_prefix('0') {
            return rest.to_string();
        }
    }
    if value < 0.0 && value > -1.0 {
        if let Some(rest) = s.strip_prefix("-0") {
            return format!("-{}", rest);
        }
    }
    s
}

/// Round a transform's data according to params.
pub fn round_transform(transform: &mut TransformItem, params: &TransformParams) {
    match transform.name.as_str() {
        "translate" => {
            transform.data = float_round(&transform.data, params);
        }
        "rotate" => {
            let mut result = Vec::new();
            result.extend(deg_round(&transform.data[..1], params));
            if transform.data.len() > 1 {
                result.extend(float_round(&transform.data[1..], params));
            }
            transform.data = result;
        }
        "skewX" | "skewY" => {
            transform.data = deg_round(&transform.data, params);
        }
        "scale" => {
            transform.data = transform_round(&transform.data, params);
        }
        "matrix" => {
            let mut result = Vec::new();
            if transform.data.len() >= 4 {
                result.extend(transform_round(&transform.data[..4], params));
            }
            if transform.data.len() > 4 {
                result.extend(float_round(&transform.data[4..], params));
            }
            transform.data = result;
        }
        _ => {}
    }
}

fn deg_round(data: &[f64], params: &TransformParams) -> Vec<f64> {
    if let Some(dp) = params.deg_precision {
        if dp >= 1 && params.float_precision < 20 {
            return smart_round(dp, data);
        }
    }
    data.iter().map(|&x| x.round()).collect()
}

fn float_round(data: &[f64], params: &TransformParams) -> Vec<f64> {
    if params.float_precision >= 1 && params.float_precision < 20 {
        smart_round(params.float_precision, data)
    } else {
        data.iter().map(|&x| x.round()).collect()
    }
}

fn transform_round(data: &[f64], params: &TransformParams) -> Vec<f64> {
    if params.transform_precision >= 1 && params.float_precision < 20 {
        smart_round(params.transform_precision, data)
    } else {
        data.iter().map(|&x| x.round()).collect()
    }
}

fn smart_round(precision: u8, data: &[f64]) -> Vec<f64> {
    let tolerance = to_fixed(0.1_f64.powi(precision as i32), precision);
    data.iter()
        .map(|&val| {
            let rounded = to_fixed(val, precision);
            if rounded != val {
                let rounded_less = to_fixed(val, precision.saturating_sub(1));
                if (rounded_less - val).abs() >= tolerance {
                    rounded
                } else {
                    rounded_less
                }
            } else {
                val
            }
        })
        .collect()
}

/// Convert a list of TransformItems to a transform string.
pub fn js2transform(transform_js: &[TransformItem], params: &TransformParams) -> String {
    transform_js
        .iter()
        .map(|t| {
            let mut t = t.clone();
            round_transform(&mut t, params);
            let data_str = cleanup_out_data(&t.data, params, t.name.as_str());
            format!("{}({})", t.name, data_str)
        })
        .collect()
}

fn cleanup_out_data(data: &[f64], params: &TransformParams, command: &str) -> String {
    let mut result = String::new();
    let mut prev: f64 = 0.0;

    for (i, &item) in data.iter().enumerate() {
        let mut delimiter = " ";

        if i == 0 {
            delimiter = "";
        }

        if params.negative_extra_space
            && delimiter != ""
            && (item < 0.0 || (item.fract() != 0.0 && prev.fract() != 0.0))
        {
            delimiter = "";
        }

        let item_str = if params.leading_zero {
            remove_leading_zero(item)
        } else {
            item.to_string()
        };

        prev = item;
        result.push_str(delimiter);
        result.push_str(&item_str);
    }
    result
}

/// Convert a matrix to the shortest equivalent list of simple transforms.
pub fn matrix_to_transform(
    orig_matrix: &TransformItem,
    params: &TransformParams,
) -> Vec<TransformItem> {
    let decompositions = get_decompositions(orig_matrix);

    let mut shortest: Option<Vec<TransformItem>> = None;
    let mut shortest_len = f64::MAX;

    for decomposition in decompositions {
        let rounded: Vec<TransformItem> = decomposition
            .iter()
            .map(|t| {
                let mut t = t.clone();
                round_transform(&mut t, params);
                t
            })
            .collect();

        let len = js2transform(&rounded, params).len() as f64;
        if len < shortest_len {
            shortest = Some(rounded);
            shortest_len = len;
        }
    }

    shortest.unwrap_or_else(|| vec![orig_matrix.clone()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform2js_translate() {
        let result = transform2js("translate(10 20)");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "translate");
        assert_eq!(result[0].data, vec![10.0, 20.0]);
    }

    #[test]
    fn test_transform2js_multiple() {
        let result = transform2js("translate(10 20) scale(2) rotate(-45)");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "translate");
        assert_eq!(result[1].name, "scale");
        assert_eq!(result[2].name, "rotate");
    }

    #[test]
    fn test_transforms_multiply_identity() {
        let transforms = vec![TransformItem {
            name: "translate".to_string(),
            data: vec![10.0, 20.0],
        }];
        let result = transforms_multiply(&transforms);
        assert_eq!(result.name, "matrix");
        assert_eq!(result.data, vec![1.0, 0.0, 0.0, 1.0, 10.0, 20.0]);
    }

    #[test]
    fn test_matrix_to_transform_identity() {
        let matrix = TransformItem {
            name: "matrix".to_string(),
            data: vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        };
        let params = TransformParams::default();
        let result = matrix_to_transform(&matrix, &params);
        // Identity matrix should decompose to something short
        let s = js2transform(&result, &params);
        assert!(s.len() <= 20);
    }
}
