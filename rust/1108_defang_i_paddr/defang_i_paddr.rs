/*
 * @Date: 2022-06-21 09:53:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:58:18
 * @FilePath: /algorithm/1108_defang_i_paddr/defang_i_paddr.rs
 */

pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

fn main() {
    assert_eq!(
        defang_i_paddr(String::from("1.1.1.1")),
        String::from("1[.]1[.]1[.]1")
    );
    assert_eq!(
        defang_i_paddr(String::from("255.100.50.0")),
        String::from("255[.]100[.]50[.]0")
    );
}
