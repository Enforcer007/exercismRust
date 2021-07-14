static VERSELAST:&str="No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
static VERSELASTBUTONE:&str="1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from(VERSELAST);
    } else if n == 1 {
        return String::from(VERSELASTBUTONE);
    } else {
        return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n - 1, if n-1==1{""} else {"s"});
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let string = (end..=start)
        .rev()
        .map(|i| verse(i))
        .fold(String::new(), |a, b| a + &b + "\n");
    string[..string.len() - 1].into()
}
