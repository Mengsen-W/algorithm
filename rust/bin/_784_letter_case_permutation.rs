/*
 * @Date: 2022-10-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-30
 * @FilePath: /algorithm/784_letter_case_permutation/letter_case_permutation.rs
 */

pub fn letter_case_permutation(s: String) -> Vec<String> {
    fn back_track(arr: &mut Vec<char>, ret: &mut Vec<String>, idx: usize) {
        if arr.len() == idx {
            ret.push(arr.iter().collect::<String>());
            return;
        }
        if arr[idx].is_ascii_digit() {
            back_track(arr, ret, idx + 1);
            return;
        }
        arr[idx] = arr[idx].to_ascii_uppercase();
        back_track(arr, ret, idx + 1);
        arr[idx] = arr[idx].to_ascii_lowercase();
        back_track(arr, ret, idx + 1);
    }
    let mut ret = vec![];
    back_track(&mut s.chars().collect::<Vec<_>>(), &mut ret, 0);
    ret
}

fn main() {
    {
        let s = String::from("a1b2");
        let ans: Vec<String> = vec!["A1B2", "A1b2", "a1B2", "a1b2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(letter_case_permutation(s), ans);
    }

    {
        let s = String::from("3z4");
        let ans: Vec<String> = vec!["3Z4", "3z4"].iter().map(|s| s.to_string()).collect();
        assert_eq!(letter_case_permutation(s), ans);
    }
}
