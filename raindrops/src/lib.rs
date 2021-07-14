pub fn raindrops(n: u32) -> String {
    let nums = [3, 5, 7];
    let sounds = ["Pling", "Plang", "Plong"];
    let zipped = nums.iter().zip(sounds.iter());
    let mut res = String::new();
    for (num, sound) in zipped {
        if n % num == 0 {
            res.push_str(sound)
        }
    }
    if res != "" {
        return res;
    } else {
        return n.to_string();
    }
}
