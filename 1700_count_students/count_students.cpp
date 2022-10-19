/*
 * @Date: 2022-10-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-19
 * @FilePath: /algorithm/1700_count_students/count_students.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int countStudents(vector<int>& students, vector<int>& sandwiches) {
    int s1 = accumulate(students.begin(), students.end(), 0);
    int s0 = students.size() - s1;
    for (int i = 0; i < sandwiches.size(); i++) {
      if (sandwiches[i] == 0 && s0 > 0) {
        s0--;
      } else if (sandwiches[i] == 1 && s1 > 0) {
        s1--;
      } else {
        break;
      }
    }
    return s0 + s1;
  }
};

int main() {
  {
    vector<int> students{1, 1, 0, 0};
    vector<int> sandwiches{0, 1, 0, 1};
    int ans = 0;
    assert(Solution().countStudents(students, sandwiches) == ans);
  }

  {
    vector<int> students{1, 1, 1, 0, 0, 1};
    vector<int> sandwiches{1, 0, 0, 0, 1, 1};
    int ans = 3;
    assert(Solution().countStudents(students, sandwiches) == ans);
  }
}