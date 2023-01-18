/*
 * @Date: 2021-05-01 10:22:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-01 11:16:25
 */

#[derive(PartialEq, Eq, Clone, Debug)]
struct Employee {
    id: i32,
    important: i32,
    subproblem: Vec<i32>,
}

impl Employee {
    fn new(id: i32, important: i32, subproblem: Vec<i32>) -> Employee {
        Employee {
            id: id,
            important: important,
            subproblem: subproblem,
        }
    }
}

fn get_important_bfs(employees: Vec<Employee>, id: i32) -> i32 {
    let mut mp: std::collections::HashMap<i32, Employee> = std::collections::HashMap::new();
    for employee in employees.iter() {
        mp.insert(employee.id, employee.clone());
    }
    let mut total = 0;
    let mut que: Vec<i32> = vec![];
    que.push(id);
    while !que.is_empty() {
        let cur_id = que[0];
        que.remove(0);
        let employee = mp[&cur_id].clone();
        total += employee.important;
        for sub_id in employee.subproblem.iter() {
            que.push(*sub_id);
        }
    }
    total
}

fn dfs(id: i32, mp: &std::collections::HashMap<i32, Employee>) -> i32 {
    let employee = mp[&id].clone();
    let mut total = employee.important;
    for sub_id in employee.subproblem.iter() {
        total += dfs(*sub_id, &mp);
    }
    total
}

fn get_important_dfs(employees: Vec<Employee>, id: i32) -> i32 {
    let mut mp: std::collections::HashMap<i32, Employee> = std::collections::HashMap::new();
    for employee in employees.iter() {
        mp.insert(employee.id, employee.clone());
    }
    dfs(id, &mp)
}

fn main() {
    let employee_1 = Employee::new(1, 5, vec![2, 3]);
    let employee_2 = Employee::new(2, 3, vec![]);
    let employee_3 = Employee::new(3, 3, vec![]);
    assert_eq!(
        get_important_bfs(
            vec![employee_1.clone(), employee_2.clone(), employee_3.clone()],
            1
        ),
        11
    );
    assert_eq!(
        get_important_dfs(
            vec![employee_1.clone(), employee_2.clone(), employee_3.clone()],
            1
        ),
        11
    );
}
