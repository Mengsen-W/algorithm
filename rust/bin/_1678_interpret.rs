/*
 * @Date: 2022-11-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-06
 * @FilePath: /algorithm/1678_interpret/interpret.rs
 */

pub fn interpret(command: String) -> String {
    let command: Vec<char> = command.chars().collect();
    (0..command.len()).fold(String::new(), |mut res, i| {
        if command[i] == 'G' {
            res.push('G');
        } else if command[i] == '(' {
            if command[i + 1] == ')' {
                res.push('o');
            } else {
                res.push_str("al");
            }
        }

        res
    })
}

fn main() {
    assert_eq!(interpret(String::from("G()(al)")), String::from("Goal"));
    assert_eq!(
        interpret(String::from("G()()()()(al)")),
        String::from("Gooooal")
    );
    assert_eq!(
        interpret(String::from("(al)G(al)()()G")),
        String::from("alGalooG")
    );
}
