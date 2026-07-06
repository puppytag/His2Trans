//! rgba color conversion module
//! Translated from rgba.c (Copyright (c) 2012 TJ Holowaychuk)

use crate::types::{rgba_t, named_color};

// ============================================================
// Named colors table (148 entries + sentinel)
// ============================================================

static NAMED_COLORS: &[named_color] = &[
    named_color { name: b"transparent", val: 0xFFFFFF00 },
    named_color { name: b"aliceblue", val: 0xF0F8FFFF },
    named_color { name: b"antiquewhite", val: 0xFAEBD7FF },
    named_color { name: b"aqua", val: 0x00FFFFFF },
    named_color { name: b"aquamarine", val: 0x7FFFD4FF },
    named_color { name: b"azure", val: 0xF0FFFFFF },
    named_color { name: b"beige", val: 0xF5F5DCFF },
    named_color { name: b"bisque", val: 0xFFE4C4FF },
    named_color { name: b"black", val: 0x000000FF },
    named_color { name: b"blanchedalmond", val: 0xFFEBCDFF },
    named_color { name: b"blue", val: 0x0000FFFF },
    named_color { name: b"blueviolet", val: 0x8A2BE2FF },
    named_color { name: b"brown", val: 0xA52A2AFF },
    named_color { name: b"burlywood", val: 0xDEB887FF },
    named_color { name: b"cadetblue", val: 0x5F9EA0FF },
    named_color { name: b"chartreuse", val: 0x7FFF00FF },
    named_color { name: b"chocolate", val: 0xD2691EFF },
    named_color { name: b"coral", val: 0xFF7F50FF },
    named_color { name: b"cornflowerblue", val: 0x6495EDFF },
    named_color { name: b"cornsilk", val: 0xFFF8DCFF },
    named_color { name: b"crimson", val: 0xDC143CFF },
    named_color { name: b"cyan", val: 0x00FFFFFF },
    named_color { name: b"darkblue", val: 0x00008BFF },
    named_color { name: b"darkcyan", val: 0x008B8BFF },
    named_color { name: b"darkgoldenrod", val: 0xB8860BFF },
    named_color { name: b"darkgray", val: 0xA9A9A9FF },
    named_color { name: b"darkgreen", val: 0x006400FF },
    named_color { name: b"darkgrey", val: 0xA9A9A9FF },
    named_color { name: b"darkkhaki", val: 0xBDB76BFF },
    named_color { name: b"darkmagenta", val: 0x8B008BFF },
    named_color { name: b"darkolivegreen", val: 0x556B2FFF },
    named_color { name: b"darkorange", val: 0xFF8C00FF },
    named_color { name: b"darkorchid", val: 0x9932CCFF },
    named_color { name: b"darkred", val: 0x8B0000FF },
    named_color { name: b"darksalmon", val: 0xE9967AFF },
    named_color { name: b"darkseagreen", val: 0x8FBC8FFF },
    named_color { name: b"darkslateblue", val: 0x483D8BFF },
    named_color { name: b"darkslategray", val: 0x2F4F4FFF },
    named_color { name: b"darkslategrey", val: 0x2F4F4FFF },
    named_color { name: b"darkturquoise", val: 0x00CED1FF },
    named_color { name: b"darkviolet", val: 0x9400D3FF },
    named_color { name: b"deeppink", val: 0xFF1493FF },
    named_color { name: b"deepskyblue", val: 0x00BFFFFF },
    named_color { name: b"dimgray", val: 0x696969FF },
    named_color { name: b"dimgrey", val: 0x696969FF },
    named_color { name: b"dodgerblue", val: 0x1E90FFFF },
    named_color { name: b"firebrick", val: 0xB22222FF },
    named_color { name: b"floralwhite", val: 0xFFFAF0FF },
    named_color { name: b"forestgreen", val: 0x228B22FF },
    named_color { name: b"fuchsia", val: 0xFF00FFFF },
    named_color { name: b"gainsboro", val: 0xDCDCDCFF },
    named_color { name: b"ghostwhite", val: 0xF8F8FFFF },
    named_color { name: b"gold", val: 0xFFD700FF },
    named_color { name: b"goldenrod", val: 0xDAA520FF },
    named_color { name: b"gray", val: 0x808080FF },
    named_color { name: b"green", val: 0x008000FF },
    named_color { name: b"greenyellow", val: 0xADFF2FFF },
    named_color { name: b"grey", val: 0x808080FF },
    named_color { name: b"honeydew", val: 0xF0FFF0FF },
    named_color { name: b"hotpink", val: 0xFF69B4FF },
    named_color { name: b"indianred", val: 0xCD5C5CFF },
    named_color { name: b"indigo", val: 0x4B0082FF },
    named_color { name: b"ivory", val: 0xFFFFF0FF },
    named_color { name: b"khaki", val: 0xF0E68CFF },
    named_color { name: b"lavender", val: 0xE6E6FAFF },
    named_color { name: b"lavenderblush", val: 0xFFF0F5FF },
    named_color { name: b"lawngreen", val: 0x7CFC00FF },
    named_color { name: b"lemonchiffon", val: 0xFFFACDFF },
    named_color { name: b"lightblue", val: 0xADD8E6FF },
    named_color { name: b"lightcoral", val: 0xF08080FF },
    named_color { name: b"lightcyan", val: 0xE0FFFFFF },
    named_color { name: b"lightgoldenrodyellow", val: 0xFAFAD2FF },
    named_color { name: b"lightgray", val: 0xD3D3D3FF },
    named_color { name: b"lightgreen", val: 0x90EE90FF },
    named_color { name: b"lightgrey", val: 0xD3D3D3FF },
    named_color { name: b"lightpink", val: 0xFFB6C1FF },
    named_color { name: b"lightsalmon", val: 0xFFA07AFF },
    named_color { name: b"lightseagreen", val: 0x20B2AAFF },
    named_color { name: b"lightskyblue", val: 0x87CEFAFF },
    named_color { name: b"lightslategray", val: 0x778899FF },
    named_color { name: b"lightslategrey", val: 0x778899FF },
    named_color { name: b"lightsteelblue", val: 0xB0C4DEFF },
    named_color { name: b"lightyellow", val: 0xFFFFE0FF },
    named_color { name: b"lime", val: 0x00FF00FF },
    named_color { name: b"limegreen", val: 0x32CD32FF },
    named_color { name: b"linen", val: 0xFAF0E6FF },
    named_color { name: b"magenta", val: 0xFF00FFFF },
    named_color { name: b"maroon", val: 0x800000FF },
    named_color { name: b"mediumaquamarine", val: 0x66CDAAFF },
    named_color { name: b"mediumblue", val: 0x0000CDFF },
    named_color { name: b"mediumorchid", val: 0xBA55D3FF },
    named_color { name: b"mediumpurple", val: 0x9370DBFF },
    named_color { name: b"mediumseagreen", val: 0x3CB371FF },
    named_color { name: b"mediumslateblue", val: 0x7B68EEFF },
    named_color { name: b"mediumspringgreen", val: 0x00FA9AFF },
    named_color { name: b"mediumturquoise", val: 0x48D1CCFF },
    named_color { name: b"mediumvioletred", val: 0xC71585FF },
    named_color { name: b"midnightblue", val: 0x191970FF },
    named_color { name: b"mintcream", val: 0xF5FFFAFF },
    named_color { name: b"mistyrose", val: 0xFFE4E1FF },
    named_color { name: b"moccasin", val: 0xFFE4B5FF },
    named_color { name: b"navajowhite", val: 0xFFDEADFF },
    named_color { name: b"navy", val: 0x000080FF },
    named_color { name: b"oldlace", val: 0xFDF5E6FF },
    named_color { name: b"olive", val: 0x808000FF },
    named_color { name: b"olivedrab", val: 0x6B8E23FF },
    named_color { name: b"orange", val: 0xFFA500FF },
    named_color { name: b"orangered", val: 0xFF4500FF },
    named_color { name: b"orchid", val: 0xDA70D6FF },
    named_color { name: b"palegoldenrod", val: 0xEEE8AAFF },
    named_color { name: b"palegreen", val: 0x98FB98FF },
    named_color { name: b"paleturquoise", val: 0xAFEEEEFF },
    named_color { name: b"palevioletred", val: 0xDB7093FF },
    named_color { name: b"papayawhip", val: 0xFFEFD5FF },
    named_color { name: b"peachpuff", val: 0xFFDAB9FF },
    named_color { name: b"peru", val: 0xCD853FFF },
    named_color { name: b"pink", val: 0xFFC0CBFF },
    named_color { name: b"plum", val: 0xDDA0DDFF },
    named_color { name: b"powderblue", val: 0xB0E0E6FF },
    named_color { name: b"purple", val: 0x800080FF },
    named_color { name: b"red", val: 0xFF0000FF },
    named_color { name: b"rosybrown", val: 0xBC8F8FFF },
    named_color { name: b"royalblue", val: 0x4169E1FF },
    named_color { name: b"saddlebrown", val: 0x8B4513FF },
    named_color { name: b"salmon", val: 0xFA8072FF },
    named_color { name: b"sandybrown", val: 0xF4A460FF },
    named_color { name: b"seagreen", val: 0x2E8B57FF },
    named_color { name: b"seashell", val: 0xFFF5EEFF },
    named_color { name: b"sienna", val: 0xA0522DFF },
    named_color { name: b"silver", val: 0xC0C0C0FF },
    named_color { name: b"skyblue", val: 0x87CEEBFF },
    named_color { name: b"slateblue", val: 0x6A5ACDFF },
    named_color { name: b"slategray", val: 0x708090FF },
    named_color { name: b"slategrey", val: 0x708090FF },
    named_color { name: b"snow", val: 0xFFFAFAFF },
    named_color { name: b"springgreen", val: 0x00FF7FFF },
    named_color { name: b"steelblue", val: 0x4682B4FF },
    named_color { name: b"tan", val: 0xD2B48CFF },
    named_color { name: b"teal", val: 0x008080FF },
    named_color { name: b"thistle", val: 0xD8BFD8FF },
    named_color { name: b"tomato", val: 0xFF6347FF },
    named_color { name: b"turquoise", val: 0x40E0D0FF },
    named_color { name: b"violet", val: 0xEE82EEFF },
    named_color { name: b"wheat", val: 0xF5DEB3FF },
    named_color { name: b"white", val: 0xFFFFFFFF },
    named_color { name: b"whitesmoke", val: 0xF5F5F5FF },
    named_color { name: b"yellow", val: 0xFFFF00FF },
    named_color { name: b"yellowgreen", val: 0x9ACD32FF },
    // Sentinel
    named_color { name: b"", val: 0 },
];

// ============================================================
// Helper: convert hex char to integer value 0-15
// ============================================================

fn h(c: u8) -> u32 {
    match c {
        b'0'..=b'9' => (c - b'0') as u32,
        b'a'..=b'f' => (c - b'a' + 10) as u32,
        b'A'..=b'F' => (c - b'A' + 10) as u32,
        _ => 0,
    }
}

// ============================================================
// Public API
// ============================================================

/// Return rgba_t from a packed u32 rgba value.
/// C: rgba_new(uint32_t rgba)
pub fn rgba_new(rgba: u32) -> rgba_t {
    rgba_t {
        r: ((rgba >> 24) & 0xff) as f64 / 255.0,
        g: ((rgba >> 16) & 0xff) as f64 / 255.0,
        b: ((rgba >> 8) & 0xff) as f64 / 255.0,
        a: (rgba & 0xff) as f64 / 255.0,
    }
}

/// Return a string representation of the color.
/// - When alpha == 1.0, returns `#RRGGBB`
/// - Otherwise returns `rgba(R, G, B, A)` with A formatted to 2 decimal places.
/// Channel values are truncated toward zero, matching C's (int) cast.
pub fn rgba_to_string(rgba: rgba_t) -> String {
    if rgba.a == 1.0 {
        format!(
            "#{:02x}{:02x}{:02x}",
            (rgba.r * 255.0) as u8,
            (rgba.g * 255.0) as u8,
            (rgba.b * 255.0) as u8
        )
    } else {
        format!(
            "rgba({}, {}, {}, {:.2})",
            (rgba.r * 255.0) as u8,
            (rgba.g * 255.0) as u8,
            (rgba.b * 255.0) as u8,
            rgba.a
        )
    }
}

/// Parse color string (e.g., "#ff0000", "rgb(255,0,0)", "red").
/// Returns Some(packed u32) on success, None on failure.
pub fn rgba_from_string(input: &[u8]) -> Option<u32> {
    if input.is_empty() {
        return None;
    }

    let s = input;

    if s[0] == b'#' {
        return rgba_from_hex_string(&s[1..]);
    }

    if s.len() >= 5 && &s[..5] == b"rgba(" {
        return rgba_from_rgba_string(s);
    }
    if s.len() >= 4 && &s[..4] == b"rgb(" {
        return rgba_from_rgb_string(s);
    }

    rgba_from_name_string(s)
}

/// Inspect the given rgba color, printing to stdout.
/// C: rgba_inspect(uint32_t rgba)
pub fn rgba_inspect(rgba: u32) {
    println!(
        "rgba({},{},{},{})",
        (rgba >> 24) & 0xff,
        (rgba >> 16) & 0xff,
        (rgba >> 8) & 0xff,
        rgba & 0xff
    );
}

// ============================================================
// Private helper: pack r,g,b,a into u32
// ============================================================

fn rgba_from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | a as u32
}

fn rgba_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    rgba_from_rgba(r, g, b, 255)
}

// ============================================================
// Private helper: parse "#RRGGBB"
// ============================================================

fn rgba_from_hex6_string(chars: &[u8]) -> u32 {
    rgba_from_rgb(
        ((h(chars[0]) << 4) + h(chars[1])) as u8,
        ((h(chars[2]) << 4) + h(chars[3])) as u8,
        ((h(chars[4]) << 4) + h(chars[5])) as u8,
    )
}

// ============================================================
// Private helper: parse "#RGB"
// ============================================================

fn rgba_from_hex3_string(chars: &[u8]) -> u32 {
    rgba_from_rgb(
        ((h(chars[0]) << 4) + h(chars[0])) as u8,
        ((h(chars[1]) << 4) + h(chars[1])) as u8,
        ((h(chars[2]) << 4) + h(chars[2])) as u8,
    )
}

// ============================================================
// Private helper: parse "rgb(r,g,b)"
// ============================================================

fn rgba_from_rgb_string(s: &[u8]) -> Option<u32> {
    // s should start with "rgb("
    let body = &s[4..];
    
    // Consume whitespace
    let body = skip_whitespace(body);
    
    // Parse r, g, b channels
    let (r, body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    let (g, body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    let (b, _body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    
    Some(rgba_from_rgb(r, g, b))
}

// ============================================================
// Private helper: parse "rgba(r,g,b,a)"
// ============================================================

fn rgba_from_rgba_string(s: &[u8]) -> Option<u32> {
    // s starts with "rgba("
    let body = &s[5..];
    
    // Consume whitespace
    let body = skip_whitespace(body);
    
    // Parse r, g, b channels
    let (r, body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    let (g, body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    let (b, body) = match parse_channel(body) {
        Some((val, rest)) => (val, rest),
        None => return None,
    };
    
    // Parse alpha as float
    let a_val = parse_alpha(body);
    
    Some(rgba_from_rgba(r, g, b, (a_val * 255.0) as u8))
}

// ============================================================
// Private helper: parse "#RRGGBB" or "#RGB"
// ============================================================

fn rgba_from_hex_string(s: &[u8]) -> Option<u32> {
    match s.len() {
        6 => Some(rgba_from_hex6_string(s)),
        3 => Some(rgba_from_hex3_string(s)),
        _ => None,
    }
}

// ============================================================
// Private helper: lookup named color value
// ============================================================

fn rgba_from_name_string(s: &[u8]) -> Option<u32> {
    for color in NAMED_COLORS {
        if color.name.is_empty() {
            break; // sentinel
        }
        if color.name == s {
            return Some(color.val);
        }
    }
    None
}

// ============================================================
// Inline helpers: skip whitespace, parse channel
// ============================================================

#[inline]
fn skip_whitespace(mut s: &[u8]) -> &[u8] {
    while !s.is_empty() && s[0] == b' ' {
        s = &s[1..];
    }
    s
}

/// Parse a decimal channel value (0-255), clamping at 255.
/// Returns (value, remaining_slice) or None on failure.
fn parse_channel(s: &[u8]) -> Option<(u8, &[u8])> {
    let mut val: u32 = 0;
    let mut pos = 0;
    let bytes = s;
    
    while pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
        val = val * 10 + (bytes[pos] - b'0') as u32;
        pos += 1;
    }
    
    if pos == 0 {
        return None;
    }
    
    if val > 255 {
        val = 255;
    }
    
    let rest = &s[pos..];
    // Skip whitespace and commas
    let rest = skip_separators(rest);
    
    Some((val as u8, rest))
}

/// Skip whitespace and comma separators.
#[inline]
fn skip_separators(mut s: &[u8]) -> &[u8] {
    while !s.is_empty() && (s[0] == b' ' || s[0] == b',') {
        s = &s[1..];
    }
    s
}

/// Parse alpha float value (0.0-1.0 or integer >=1 means 1.0).
fn parse_alpha(s: &[u8]) -> f64 {
    let s = skip_whitespace(s);
    if s.is_empty() {
        return 0.0;
    }
    
    // If first char is >= '1' and <= '9', alpha = 1.0
    if s[0] >= b'1' && s[0] <= b'9' {
        return 1.0;
    }
    
    let mut pos = 0;
    // Skip leading '0'
    if pos < s.len() && s[pos] == b'0' {
        pos += 1;
    }
    
    let mut alpha: f64 = 0.0;
    if pos < s.len() && s[pos] == b'.' {
        pos += 1;
        let mut n: f64 = 0.1;
        while pos < s.len() && s[pos] >= b'0' && s[pos] <= b'9' {
            alpha += ((s[pos] - b'0') as f64) * n;
            n *= 0.1;
            pos += 1;
        }
    }
    
    alpha
}
