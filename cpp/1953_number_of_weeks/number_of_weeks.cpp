/*
 * @Date: 2024-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-16
 * @FilePath: /algorithm/cpp/1953_number_of_weeks/number_of_weeks.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long numberOfWeeks(vector<int>& milestones) {
    // 耗时最长工作所需周数
    long long longest = *max_element(milestones.begin(), milestones.end());
    // 其余工作共计所需周数
    long long rest = accumulate(milestones.begin(), milestones.end(), 0LL) - longest;
    if (longest > rest + 1) {
      // 此时无法完成所耗时最长的工作
      return rest * 2 + 1;
    } else {
      // 此时可以完成所有工作
      return longest + rest;
    }
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{1, 2, 3}, 6},
      {{5, 2, 1}, 7},
  };

  for (auto& [milestones, ans] : tests) {
    assert(Solution().numberOfWeeks(milestones) == ans);
  }
}