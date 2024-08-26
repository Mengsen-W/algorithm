/*
 * @Date: 2021-05-01 10:22:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-01 10:41:56
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Employee {
 public:
  int id;
  int importance;
  vector<int> subordinates;
};

class Solution {
 public:
  unordered_map<int, Employee *> mp;

  int dfs(int id) {
    Employee *employee = mp[id];
    int total = employee->importance;
    for (int subId : employee->subordinates) {
      total += dfs(subId);
    }
    return total;
  }

  int getImportance(vector<Employee *> employees, int id) {
    for (auto &employee : employees) {
      mp[employee->id] = employee;
    }
    return dfs(id);
  }
};

int main() {
  vector<tuple<vector<Employee *>, int, int>> tests{
      {{new Employee{1, 5, {2, 3}}, new Employee{2, 3, {}}, new Employee{3, 3, {}}}, 1, 11},
      {{new Employee{1, 2, {5}}, new Employee{5, -3, {}}}, 5, -3},
  };

  for (auto& [employees, id, ans] : tests) {
    assert(Solution().getImportance(employees, id) == ans);
  }

  return 0;
}