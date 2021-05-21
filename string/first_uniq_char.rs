use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut map = HashMap::new();
    let s_b = s.as_bytes();

    for i in 0..s_b.len() {
        let val = s_b[i];

        match map.get_mut(&val) {
            Some(x) => {
                *x += 1;
            },
            None => {
                map.insert(val, 1);
            }
        }
    }
    for i in 0..s_b.len() {
        let val = s_b[i];
        match map.get(&val) {
            None => (),
            Some(x) => {
                if *x == 1 {
                    return i as i32;
                }
            }
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use crate::first_uniq_char::first_uniq_char;

    #[test]
    fn it_work_1() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    }
}