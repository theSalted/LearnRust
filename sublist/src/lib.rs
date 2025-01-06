#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal
    }

    if superlist(_first_list, _second_list) {
        return Comparison::Superlist
    } else if superlist(_second_list, _first_list) {
        return Comparison::Sublist
    }

    return Comparison::Unequal
}

pub fn superlist<T: PartialEq>(super_list: &[T], sub_list: &[T]) -> bool {
    let range = sub_list.len();
    
    for i in 0..super_list.len() {
        if let Some(slice) = super_list.get(i..(i+range)) {
            if slice == sub_list {
                return true
            }
        } else {
            return false
        }

    }

    return false
}
