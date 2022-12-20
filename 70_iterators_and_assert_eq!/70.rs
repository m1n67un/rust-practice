fn main() {
    let my_vec = vec!['a', 'b', '거', '姜'];
    let mut my_vec_iter = my_vec.iter();
    println!("{}", my_vec_iter.next());

    assert_eq!(my_vec.iter().next(), Some(&'a'));
    assert_eq!(my_vec.iter().next(), Some(&'b'));
    assert_eq!(my_vec.iter().next(), Some(&'거'));
    assert_eq!(my_vec.iter().next(), Some(&'姜'));
    assert_eq!(my_vec.iter().next(), None);
}