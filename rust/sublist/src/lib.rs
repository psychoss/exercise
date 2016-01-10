#[derive(PartialEq,Eq,Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if contains(a, b) {
        Comparison::Superlist
    } else if contains(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    }
    if a.starts_with(b) {
        return true;
    }
    contains(&a[1..], b)

}

#[test]
fn test_empty_equal() {
    let v: &[u8] = &[];
    assert_eq!(Comparison::Equal, sublist(&v, &v));
}
#[test]
fn test_contains() {
    let v_a: Vec<u8> = (1..100).collect();
    let v_b: Vec<u8> = (1..50).collect();

    assert_eq!(Comparison::Superlist, sublist(&v_a, &v_b));
}
#[test]
#[ignore]
fn recurring_values_sublist() {
    assert_eq!(Comparison::Sublist,
               sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1]));
}

fn main() {
    a();
}
fn a() {

    let x = match c() {
        Some(v) => v,
        None => {
            return;
        }
    };
    b(x);
}
fn b(n: u8) {
    println!("{}", n);
}
fn c() -> Option<u8> {
    None
}
