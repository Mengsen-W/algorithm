/*
 * @Date: 2021-05-01 10:22:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-01 10:41:56
 */

#include <cassert>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

class Employee {
 public:
  int id;
  int importance;
  vector<int> subordinates;
};

unordered_map<int, Employee *> mp;

int dfs(int id) {
  Employee *employee = mp[id];
  int total = employee->importance;
  for (int subId : employee->subordinates) {
    total += dfs(subId);
  }
  return total;
}

int get_importance_dfs(vector<Employee *> employees, int id) {
  for (auto &employee : employees) {
    mp[employee->id] = employee;
  }
  return dfs(id);
}

int get_importance_bfs(vector<Employee *> employees, int id) {
  unordered_map<int, Employee *> mp;
  for (auto &employee : employees) {
    mp[employee->id] = employee;
  }

  int total = 0;
  queue<int> que;
  que.push(id);
  while (!que.empty()) {
    int curId = que.front();
    que.pop();
    Employee *employee = mp[curId];
    total += employee->importance;
    for (int subId : employee->subordinates) {
      que.push(subId);
    }
  }
  return total;
}

int main() {
  Employee *employee_1 = new Employee();
  employee_1->id = 1;
  employee_1->importance = 5;
  employee_1->subordinates = vector<int>{2, 3};
  Employee *employee_2 = new Employee();
  employee_2->id = 2;
  employee_2->importance = 3;
  employee_2->subordinates = vector<int>{};
  Employee *employee_3 = new Employee();
  employee_3->id = 3;
  employee_3->importance = 3;
  employee_3->subordinates = vector<int>{};
  vector<Employee *> employees{employee_1, employee_2, employee_3};

  assert(get_importance_bfs(employees, 1) == 11);
  assert(get_importance_dfs(employees, 1) == 11);

  return 0;
}