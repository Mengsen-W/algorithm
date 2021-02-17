/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:02:22
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:33:34
 */

fn average(salary: Vec<i32>) -> f64 {
    (salary.iter().sum::<i32>() - salary.iter().min().unwrap() - salary.iter().max().unwrap())
        as f64
        / (salary.len() - 2) as f64
}

fn main() {
    let mut salary: Vec<i32>;
    salary = vec![4000, 3000, 1000, 2000];
    println!("{}", average(salary.clone()));
    salary = vec![1000, 2000, 3000];
    println!("{}", average(salary.clone()));
    salary = vec![6000, 5000, 4000, 3000, 2000, 1000];
    println!("{}", average(salary.clone()));
    salary = vec![8000, 9000, 2000, 3000, 6000, 1000];
    println!("{}", average(salary.clone()));
}
