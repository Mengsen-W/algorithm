/*
 * @Date: 2021-07-24 13:31:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-24 13:38:44
 */

fn maximum_time(time: String) -> String {
    let mut ans: Vec<char> = time.chars().collect();

    ans[0] = if ans[0] == '?' {
        if ans[1] == '?' || ans[1] <= '3' {
            '2'
        } else {
            '1'
        }
    } else {
        ans[0]
    };
    ans[1] = if ans[1] == '?' {
        if ans[0] == '2' {
            '3'
        } else {
            '9'
        }
    } else {
        ans[1]
    };
    ans[3] = if ans[3] == '?' { '5' } else { ans[3] };
    ans[4] = if ans[4] == '?' { '9' } else { ans[4] };

    ans.into_iter().collect()
}

fn main() {
    {
        let time = "2?:?0".to_string();
        assert!(maximum_time(time) == "23:50".to_string());
    }

    {
        let time = "0?:3?".to_string();
        assert!(maximum_time(time) == "09:39".to_string());
    }

    {
        let time = "1?:22".to_string();
        assert!(maximum_time(time) == "19:22".to_string());
    }
}
