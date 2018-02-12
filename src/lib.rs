use std::cmp;

#[derive(Debug, PartialEq)]
pub enum DiffResult<T: PartialEq + Clone> {
    Removed(DiffElement<T>),
    Common(DiffElement<T>),
    Added(DiffElement<T>),
}

#[derive(Debug, PartialEq)]
pub struct DiffElement<T: PartialEq + Clone> {
    old_index: Option<usize>,
    new_index: Option<usize>,
    data: T,
}

fn create_table<T: PartialEq + Clone>(old: &[T], new: &[T]) -> Vec<Vec<u32>> {
    let new_len = new.len();
    let old_len = old.len();
    let mut table = vec![vec![0; old_len + 1]; new_len + 1];
    for (n, _) in new.iter().enumerate() {
        let n = new_len - n - 1;
        table[n][old_len] = 0;
        for (o, _) in old.iter().enumerate() {
            let o = old_len - o - 1;
            if new[n] == old[o] {
                table[n][o] = table[n + 1][o + 1] + 1;
            } else {
                table[n][o] = cmp::max(table[n + 1][o], table[n][o + 1]);
            }
        }
    }
    table
}

pub fn diff<T: PartialEq + Clone>(old: &[T], new: &[T]) -> Vec<DiffResult<T>> {
    let table = create_table(old, new);
    let mut n = 0;
    let mut o = 0;
    let mut result: Vec<DiffResult<T>> = Vec::new();
    let new_len = new.len();
    let old_len = old.len();

    loop {
        if n >= new_len || o >= old_len {
            break;
        }
        if new[n] == old[o] {
            result.push(DiffResult::Common(DiffElement {
                                               old_index: Some(o),
                                               new_index: Some(n),
                                               data: new[n].clone(),
                                           }));
            n += 1;
            o += 1;
        } else if table[n + 1][o] >= table[n][o + 1] {
            result.push(DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(n),
                                              data: new[n].clone(),
                                          }));
            n += 1;
        } else {
            result.push(DiffResult::Removed(DiffElement {
                                                old_index: Some(o),
                                                new_index: None,
                                                data: old[o].clone(),
                                            }));
            o += 1;
        }
    }
    while n < new_len {
        result.push(DiffResult::Added(DiffElement {
                                          old_index: None,
                                          new_index: Some(n),
                                          data: new[n].clone(),
                                      }));
        n += 1;
    }
    while o < old_len {
        result.push(DiffResult::Removed(DiffElement {
                                            old_index: Some(o),
                                            new_index: None,
                                            data: old[o].clone(),
                                        }));
        o += 1;
    }
    result
}


#[test]
fn shoud_create_table_with_numbers() {
    let table = create_table(&vec![2, 3], &vec![0, 1, 2]);
    let expected = vec![vec![1, 0, 0], vec![1, 0, 0], vec![1, 0, 0], vec![0, 0, 0]];
    assert_eq!(table, expected);
}

#[test]
fn shoud_create_table_with_char() {
    let table = create_table(&vec!["a", "d"], &vec!["a", "b", "c"]);
    let expected = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(table, expected);
}

#[test]
fn shoud_create_table_with_string() {
    let table = create_table(&vec!["abc", "bcd"], &vec!["abc", "bcd", "c"]);
    let expected = vec![vec![2, 1, 0], vec![1, 1, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(table, expected);
}

#[test]
fn shoud_create_diff_result_with_added() {
    let result = diff(&vec!["abc", "c"], &vec!["abc", "bcd", "c"]);
    let expected = vec![DiffResult::Common(DiffElement {
                                               old_index: Some(0),
                                               new_index: Some(0),
                                               data: "abc",
                                           }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(1),
                                              data: "bcd",
                                          }),
                        DiffResult::Common(DiffElement {
                                               old_index: Some(1),
                                               new_index: Some(2),
                                               data: "c",
                                           })];

    assert_eq!(result, expected);
}


#[test]
fn shoud_create_diff_result_with_removed() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec!["abc", "c"]);
    let expected = vec![DiffResult::Common(DiffElement {
                                               old_index: Some(0),
                                               new_index: Some(0),
                                               data: "abc",
                                           }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "bcd",
                                            }),
                        DiffResult::Common(DiffElement {
                                               old_index: Some(2),
                                               new_index: Some(1),
                                               data: "c",
                                           })];
    assert_eq!(result, expected);
}
