/*
 * @Date: 2023-02-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-18
 * @FilePath: /algorithm/rust/1237_find_solution/find_solution.rs
 */

struct CustomFunction;
impl CustomFunction {
    pub fn f(x: i32, y: i32) -> i32 {}
}

pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let (mut x, mut y) = (1, 1000);
    while x <= 1000 && y >= 1 {
        while y >= 1 && customfunction.f(x, y) > z {
            y -= 1;
        }
        if y >= 1 && customfunction.f(x, y) == z {
            res.push(vec![x, y]);
        }
        x += 1;
    }
    res
}
