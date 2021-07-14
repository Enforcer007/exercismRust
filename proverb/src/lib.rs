pub fn build_proverb(list: &[&str]) -> String {
    let n = list.len();
    if n == 0 {
        return String::new();
    }
    let mut x = (0..n - 1)
        .map(|x| format!("For want of a {} the {} was lost.", &list[x], &list[x + 1]))
        .collect::<Vec<String>>();
    x.push(format!("And all for the want of a {}.", list[0]));
    x.join("\n")
}
