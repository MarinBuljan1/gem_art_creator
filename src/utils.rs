pub fn to_excel_column(num: usize) -> String {
    let mut result = String::new();
    let mut n = num;
    while n > 0 {
        let remainder = (n - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        n = (n - 1) / 26;
    }
    result
}

pub fn expand_shorthand_hex(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 3 {
        let mut expanded = String::with_capacity(6);
        for char in hex.chars() {
            expanded.push(char);
            expanded.push(char);
        }
        expanded
    } else {
        hex.to_string()
    }
}