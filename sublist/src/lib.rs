#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (big_list, small_list, swapped) = if _first_list.len() > _second_list.len() {
        (_first_list, _second_list, false)
    } else {
        (_second_list, _first_list, true)
    };
    let small_n = small_list.len();
    let big_n = big_list.len();
    let subset = match small_n != 0 {
        true => big_list.windows(small_n).any(|x| x == small_list),
        false => true,
    };

    match (subset, swapped, big_n == small_n) {
        (true, false, false) => Comparison::Superlist,
        (true, true, false) => Comparison::Sublist,
        (true, _, true) => Comparison::Equal,
        (false, _, _) => Comparison::Unequal,
    }
}
