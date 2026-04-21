/*
 * @Date: 2021-10-11 08:51:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-11 09:37:28
 */

struct Solution;

use TreeNode::*;

const G: i32 = 1_000_000_000;
const M: i32 = 1_000_000;
const K: i32 = 1_000;
const D: i32 = 100;
const C: i32 = 10;

enum TreeNode {
    Cons(Box<TreeNode>, Box<TreeNode>),

    Billion(i32),
    Million(i32),
    Thousand(i32),
    Hundred(i32),
    TwentyPlus(i32),
    OneToTwenty(i32),
    Zero,
}

fn gen(n: i32) -> Box<TreeNode> {
    match n {
        0 => Box::new(Zero),
        n if n < 20 => Box::new(OneToTwenty(n)),
        n if n < D => Box::new(Cons(
            Box::new(TwentyPlus(n / C)),
            Box::new(OneToTwenty(n % C)),
        )),
        n if n < K => Box::new(Cons(
            Box::new(Cons(Box::new(OneToTwenty(n / D)), Box::new(Hundred(n / D)))),
            gen(n % D),
        )),
        n if n < M => Box::new(Cons(
            Box::new(Cons(gen(n / K), Box::new(Thousand(n / K)))),
            gen(n % K),
        )),
        n if n < G => Box::new(Cons(
            Box::new(Cons(gen(n / M), Box::new(Million(n / M)))),
            gen(n % M),
        )),
        n if n <= i32::max_value() => Box::new(Cons(
            Box::new(Cons(gen(n / G), Box::new(Billion(n / G)))),
            gen(n % G),
        )),
        _ => Box::new(Zero),
    }
}

impl TreeNode {
    fn to_string(&self) -> String {
        match self {
            Cons(lhs, rhs) => {
                format!("{}{}", lhs.to_string(), rhs.to_string(),)
            }
            Billion(x) => (if *x == 0 { "" } else { "Billion " }).to_owned(),
            Million(x) => (if *x == 0 { "" } else { "Million " }).to_owned(),
            Thousand(x) => (if *x == 0 { "" } else { "Thousand " }).to_owned(),
            Hundred(x) => (if *x == 0 { "" } else { "Hundred " }).to_owned(),
            TwentyPlus(x) => (match *x {
                2 => "Twenty ",
                3 => "Thirty ",
                4 => "Forty ",
                5 => "Fifty ",
                6 => "Sixty ",
                7 => "Seventy ",
                8 => "Eighty ",
                9 => "Ninety ",
                _ => "",
            })
            .to_owned(),
            OneToTwenty(x) => (match *x {
                1 => "One ",
                2 => "Two ",
                3 => "Three ",
                4 => "Four ",
                5 => "Five ",
                6 => "Six ",
                7 => "Seven ",
                8 => "Eight ",
                9 => "Nine ",
                10 => "Ten ",
                11 => "Eleven ",
                12 => "Twelve ",
                13 => "Thirteen ",
                14 => "Fourteen ",
                15 => "Fifteen ",
                16 => "Sixteen ",
                17 => "Seventeen ",
                18 => "Eighteen ",
                19 => "Nineteen ",
                _ => "",
            })
            .to_owned(),
            Zero => "".to_owned(),
        }
    }
}

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            "Zero".to_owned()
        } else {
            gen(num).to_string().trim_end().to_owned()
        }
    }
}

fn main() {
    assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
    assert_eq!(
        Solution::number_to_words(12345),
        "Twelve Thousand Three Hundred Forty Five"
    );
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
    // assert_eq!(Solution::number_to_words(1231234567891) ,
    //      "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One");
}
