/*
 * @Date: 2022-10-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-05
 * @FilePath: /algorithm/811_subdomain_visits/subdomain_visits.rs
 */

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut m = std::collections::HashMap::new();

    for i in cpdomains.iter() {
        let split = i.split(" ").collect::<Vec<&str>>();
        let num = split[0].parse::<i32>().unwrap();
        let url = split[1];

        m.entry(String::from(url))
            .and_modify(|x| *x += num)
            .or_insert(num);

        for (index, v) in url.as_bytes().iter().enumerate() {
            if *v == '.' as u8 {
                m.entry(url[index + 1..].to_string())
                    .and_modify(|x| *x += num)
                    .or_insert(num);
            }
        }
    }

    m.iter().map(|(x, y)| format!("{} {}", y, x)).collect()
}

fn main() {
    {
        let cpdomains: Vec<String> = vec!["9001 discuss.leetcode.com"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans: Vec<String> = vec!["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(subdomain_visits(cpdomains), ans);
    }

    {
        let cpdomains: Vec<String> = vec![
            "900 google.mail.com",
            "50 yahoo.com",
            "1 intel.mail.com",
            "5 wiki.org",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let ans: Vec<String> = vec![
            "901 mail.com",
            "50 yahoo.com",
            "900 google.mail.com",
            "5 wiki.org",
            "5 org",
            "1 intel.mail.com",
            "951 com",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(subdomain_visits(cpdomains), ans);
    }
}
