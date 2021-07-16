pub fn brackets_are_balanced(string: &str) -> bool {
    let opening_braces: [char; 3] = ['{', '(', '['];
    let closing_braces: [char; 3] = ['}', ')', ']'];
    let mut buffer = Vec::<char>::new();
    for i in string.chars() {
        if opening_braces.contains(&i) {
            buffer.push(i);
        } else if closing_braces.contains(&i) {
            let opposite_brace = match i {
                '}' => '{',
                ')' => '(',
                _ => '[',
            };
            if let Some(x) = buffer.last() {
                if opposite_brace == *x {
                    buffer.pop();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    buffer.len() == 0
}
