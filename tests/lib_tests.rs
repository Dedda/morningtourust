use morningtourust;

#[test]
fn download_word_list() {
    let word_list = morningtourust::download_word_list().unwrap();
    assert_ne!(0, word_list.pre.len());
    assert_ne!(0, word_list.post.len());
}

#[test]
fn combination() {
    let word_list = morningtourust::download_word_list().unwrap();
    let combination = morningtourust::combination(&word_list);
    assert!(combination.is_some());
    assert_ne!(0, combination.unwrap().len())
}