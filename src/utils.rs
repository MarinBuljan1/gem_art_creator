pub fn to_excel_column(num: usize) -> String {
    let mut s = String::new();
    let mut n = num;
    while n > 0 {
        let rem = (n - 1) % 26;
        s.insert(0, (b'A' + rem as u8) as char);
        n = (n - 1) / 26;
    }
    s
}
