//! Path data parser and serializer.
//!
//! Ported from SVGO's lib/path.js.
//! Based on https://www.w3.org/TR/SVG11/paths.html#PathDataBNF.

/// A single path segment: a command character and its numeric arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct PathSeg {
    /// Command byte: b'M', b'L', b'H', b'V', b'C', b'S', b'Q', b'T', b'A', b'Z' (and lowercase).
    pub command: u8,
    /// Numeric arguments for this command.
    pub args: Vec<f64>,
}

/// Number of arguments per path command.
fn args_count_per_command(command: u8) -> Option<usize> {
    match command {
        b'M' | b'm' => Some(2),
        b'Z' | b'z' => Some(0),
        b'L' | b'l' => Some(2),
        b'H' | b'h' => Some(1),
        b'V' | b'v' => Some(1),
        b'C' | b'c' => Some(6),
        b'S' | b's' => Some(4),
        b'Q' | b'q' => Some(4),
        b'T' | b't' => Some(2),
        b'A' | b'a' => Some(7),
        _ => None,
    }
}

fn is_command(c: u8) -> bool {
    args_count_per_command(c).is_some()
}

fn is_white_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || b'\r' == c || c == b'\n'
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

/// Read a single floating-point number from `string` starting at `cursor`.
/// Returns `(new_cursor, Some(number))` or `(cursor, None)` if no number found.
fn read_number(string: &[u8], cursor: usize) -> (usize, Option<f64>) {
    let mut i = cursor;
    let mut value = String::new();
    // state: none, sign, whole, decimal_point, decimal, e, exponent_sign, exponent
    let mut state: u8 = 0; // 0=none, 1=sign, 2=whole, 3=decimal_point, 4=decimal, 5=e, 6=exponent_sign, 7=exponent

    while i < string.len() {
        let c = string[i];
        if c == b'+' || c == b'-' {
            if state == 0 {
                state = 1;
                value.push(c as char);
                i += 1;
                continue;
            }
            if state == 5 {
                state = 6;
                value.push(c as char);
                i += 1;
                continue;
            }
        }
        if is_digit(c) {
            if state <= 2 {
                state = 2;
                value.push(c as char);
                i += 1;
                continue;
            }
            if state == 3 || state == 4 {
                state = 4;
                value.push(c as char);
                i += 1;
                continue;
            }
            if state >= 5 && state <= 7 {
                state = 7;
                value.push(c as char);
                i += 1;
                continue;
            }
        }
        if c == b'.' {
            if state <= 2 {
                state = 3;
                value.push(c as char);
                i += 1;
                continue;
            }
        }
        if c == b'E' || c == b'e' {
            if state == 2 || state == 3 || state == 4 {
                state = 5;
                value.push(c as char);
                i += 1;
                continue;
            }
        }
        break;
    }

    if let Ok(number) = value.parse::<f64>() {
        if number.is_nan() {
            (cursor, None)
        } else {
            (i - 1, Some(number))
        }
    } else {
        (cursor, None)
    }
}

/// Parse SVG path data string into a vector of path segments.
pub fn parse_path_data(string: &str) -> Vec<PathSeg> {
    let bytes = string.as_bytes();
    let mut path_data: Vec<PathSeg> = Vec::new();
    let mut command: Option<u8> = None;
    let mut args: Vec<f64> = Vec::new();
    let mut args_count: usize = 0;
    let mut can_have_comma = false;
    let mut had_comma = false;
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i];
        if is_white_space(c) {
            i += 1;
            continue;
        }
        if can_have_comma && c == b',' {
            if had_comma {
                break;
            }
            had_comma = true;
            i += 1;
            continue;
        }
        if is_command(c) {
            if had_comma {
                break;
            }
            if command.is_none() {
                // moveto should be leading command
                if c != b'M' && c != b'm' {
                    break;
                }
            } else if !args.is_empty() {
                // previous command arguments not flushed
                break;
            }
            command = Some(c);
            args = Vec::new();
            args_count = args_count_per_command(c).unwrap_or(0);
            can_have_comma = false;
            if args_count == 0 {
                path_data.push(PathSeg {
                    command: c,
                    args: Vec::new(),
                });
            }
            i += 1;
            continue;
        }
        if command.is_none() {
            break;
        }
        // read next argument
        let new_cursor;
        let number;
        let cmd = command.unwrap();
        if cmd == b'A' || cmd == b'a' {
            let position = args.len();
            if position == 0 || position == 1 || position == 2 || position == 5 || position == 6 {
                let (nc, n) = read_number(bytes, i);
                new_cursor = nc;
                number = n;
            } else if position == 3 || position == 4 {
                // read flags
                if c == b'0' {
                    new_cursor = i;
                    number = Some(0.0);
                } else if c == b'1' {
                    new_cursor = i;
                    number = Some(1.0);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            let (nc, n) = read_number(bytes, i);
            new_cursor = nc;
            number = n;
        }
        let number = match number {
            Some(n) => n,
            None => break,
        };
        args.push(number);
        can_have_comma = true;
        had_comma = false;
        i = new_cursor + 1;
        if args.len() == args_count {
            if cmd == b'A' || cmd == b'a' {
                args[0] = args[0].abs();
                args[1] = args[1].abs();
            }
            path_data.push(PathSeg {
                command: cmd,
                args: args.clone(),
            });
            // subsequent moveto coordinates are treated as implicit lineto
            if cmd == b'M' {
                command = Some(b'L');
                args_count = 2;
            } else if cmd == b'm' {
                command = Some(b'l');
                args_count = 2;
            }
            args.clear();
        }
    }
    path_data
}

/// Remove leading zero from a float string: 0.5 -> .5, -0.5 -> -.5
fn remove_leading_zero(value: f64) -> String {
    let s = value.to_string();
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

/// Round a number to the given precision and stringify.
fn round_and_stringify(number: f64, precision: Option<u8>) -> (String, f64) {
    let rounded = match precision {
        Some(p) => {
            let pow = 10_f64.powi(p as i32);
            (number * pow).round() / pow
        }
        None => number,
    };
    (remove_leading_zero(rounded), rounded)
}

/// Stringify path segment arguments, producing the shortest form.
fn stringify_args(
    command: u8,
    args: &[f64],
    precision: Option<u8>,
    disable_space_after_flags: bool,
) -> String {
    let mut result = String::new();
    let mut previous: f64 = 0.0;
    let mut first = true;

    for (i, &arg) in args.iter().enumerate() {
        let (rounded_str, rounded) = round_and_stringify(arg, precision);

        if disable_space_after_flags
            && (command == b'A' || command == b'a')
            && (i % 7 == 4 || i % 7 == 5)
        {
            result.push_str(&rounded_str);
        } else if first || rounded < 0.0 {
            result.push_str(&rounded_str);
        } else if previous.fract() != 0.0 && rounded_str.starts_with('.') {
            // remove space before decimal with zero whole when previous is also decimal
            result.push_str(&rounded_str);
        } else {
            result.push(' ');
            result.push_str(&rounded_str);
        }
        previous = rounded;
        first = false;
    }
    result
}

/// Stringify path data segments into an SVG path data string.
///
/// Produces the shortest form by:
/// - Omitting separators before negatives/decimals
/// - Combining consecutive same-command segments
pub fn stringify_path_data(path_data: &[PathSeg], precision: Option<u8>) -> String {
    if path_data.len() == 1 {
        let seg = &path_data[0];
        return format!(
            "{}{}",
            seg.command as char,
            stringify_args(seg.command, &seg.args, precision, false)
        );
    }

    let mut result = String::new();
    let mut prev = path_data[0].clone();

    // match leading moveto with following lineto
    if path_data[1].command == b'L' {
        prev.command = b'M';
    } else if path_data[1].command == b'l' {
        prev.command = b'm';
    }

    for i in 1..path_data.len() {
        let seg = &path_data[i];
        let should_combine =
            (prev.command == seg.command && prev.command != b'M' && prev.command != b'm')
                || (prev.command == b'M' && seg.command == b'L')
                || (prev.command == b'm' && seg.command == b'l');

        if should_combine {
            prev.args.extend_from_slice(&seg.args);
            if i == path_data.len() - 1 {
                result.push(prev.command as char);
                result.push_str(&stringify_args(prev.command, &prev.args, precision, false));
            }
        } else {
            result.push(prev.command as char);
            result.push_str(&stringify_args(prev.command, &prev.args, precision, false));

            if i == path_data.len() - 1 {
                result.push(seg.command as char);
                result.push_str(&stringify_args(seg.command, &seg.args, precision, false));
            } else {
                prev = seg.clone();
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let segs = parse_path_data("M 10,20 L 30,40");
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].command, b'M');
        assert_eq!(segs[0].args, vec![10.0, 20.0]);
        assert_eq!(segs[1].command, b'L');
        assert_eq!(segs[1].args, vec![30.0, 40.0]);
    }

    #[test]
    fn test_parse_relative() {
        let segs = parse_path_data("m10 20l-.5.5");
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].command, b'm');
        assert_eq!(segs[0].args, vec![10.0, 20.0]);
        assert_eq!(segs[1].command, b'l');
        assert_eq!(segs[1].args, vec![-0.5, 0.5]);
    }

    #[test]
    fn test_parse_arc_flags() {
        // a1 1 0 0130 30 -> flags 0, 0, 1 then coords 30, 30
        let segs = parse_path_data("a1 1 0 0 1 30 30");
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].command, b'a');
        assert_eq!(segs[0].args, vec![1.0, 1.0, 0.0, 0.0, 1.0, 30.0, 30.0]);
    }

    #[test]
    fn test_parse_arc_flags_packed() {
        // a1 1 0 0130 30 -> flags 0 0 1 then 30 30
        let segs = parse_path_data("a1 1 0 0130 30");
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].command, b'a');
        assert_eq!(segs[0].args, vec![1.0, 1.0, 0.0, 0.0, 1.0, 30.0, 30.0]);
    }

    #[test]
    fn test_stringify_shortest() {
        let segs = vec![
            PathSeg {
                command: b'M',
                args: vec![0.0, 0.0],
            },
            PathSeg {
                command: b'l',
                args: vec![0.5, -0.5],
            },
        ];
        let result = stringify_path_data(&segs, None);
        assert_eq!(result, "M0 0l.5-.5");
    }

    #[test]
    fn test_stringify_single() {
        let segs = vec![PathSeg {
            command: b'M',
            args: vec![10.0, 20.0],
        }];
        let result = stringify_path_data(&segs, None);
        assert_eq!(result, "M10 20");
    }

    #[test]
    fn test_round_trip() {
        let input = "M 10 20 L 30 40";
        let segs = parse_path_data(input);
        let output = stringify_path_data(&segs, None);
        assert_eq!(output, "M10 20L30 40");
    }

    #[test]
    fn test_remove_leading_zero() {
        assert_eq!(remove_leading_zero(0.5), ".5");
        assert_eq!(remove_leading_zero(-0.5), "-.5");
        assert_eq!(remove_leading_zero(1.5), "1.5");
        assert_eq!(remove_leading_zero(0.0), "0");
    }
}
