#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

trait SublistSearcher<T> {
    fn contains_slice(&self, sublist: &[T]) -> bool;
}

impl<T> SublistSearcher<T> for [T]
where
    T: PartialEq,
{
    fn contains_slice(&self, sublist: &[T]) -> bool {
        self.is_empty() && sublist.is_empty()
            || (!sublist.is_empty() && self.windows(sublist.len()).any(|x| x == sublist))
    }
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (len_1, len_2) if len_1 == len_2 && first_list == second_list => Comparison::Equal,
        (len_1, len_2)
            if second_list.is_empty() && !first_list.is_empty()
                || (len_1 > len_2 && first_list.contains_slice(second_list)) =>
        {
            Comparison::Superlist
        }
        (len_1, len_2)
            if first_list.is_empty() && !second_list.is_empty()
                || (len_1 < len_2 && second_list.contains_slice(first_list)) =>
        {
            Comparison::Sublist
        }
        _ => Comparison::Unequal,
    }
}
