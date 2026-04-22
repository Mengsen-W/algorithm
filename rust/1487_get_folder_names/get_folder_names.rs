/*
 * @Date: 2023-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-03
 * @FilePath: /algorithm/rust/1487_get_folder_names/get_folder_names.rs
 */

pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut d = std::collections::HashMap::new();
    names
        .into_iter()
        .map(|name| {
            let mut s = name.clone();
            while d.contains_key(&s) {
                s = format!("{}({})", name, d[&name]);
                *d.get_mut(&name).unwrap() += 1;
            }
            d.insert(s.clone(), 1);
            s
        })
        .collect()
}

fn main() {
    {
        let names = vec!["pes", "fifa", "gta", "pes(2019)"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["pes", "fifa", "gta", "pes(2019)"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(get_folder_names(names), ans);
    }

    {
        let names = vec!["gta", "gta(1)", "gta", "avalon"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["gta", "gta(1)", "gta(2)", "avalon"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(get_folder_names(names), ans);
    }

    {
        let names = vec![
            "onepiece",
            "onepiece(1)",
            "onepiece(2)",
            "onepiece(3)",
            "onepiece",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans: Vec<String> = vec![
            "onepiece",
            "onepiece(1)",
            "onepiece(2)",
            "onepiece(3)",
            "onepiece(4)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(get_folder_names(names), ans);
    }

    {
        let names = vec!["wano", "wano", "wano", "wano"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["wano", "wano(1)", "wano(2)", "wano(3)"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(get_folder_names(names), ans);
    }

    {
        let names = vec!["kaido", "kaido(1)", "kaido", "kaido(1)"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(get_folder_names(names), ans);
    }
}
