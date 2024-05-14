/*
 * @Date: 2024-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-14
 * @FilePath: /algorithm/cpp/2244_minimum_rounds/minimum_rounds.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumRounds(vector<int>& tasks) {
    unordered_map<int, int> cnt;
    for (int t : tasks) {
      cnt[t]++;
    }
    int res = 0;
    for (auto [_, v] : cnt) {
      if (v == 1) {
        return -1;
      } else if (v % 3 == 0) {
        res += v / 3;
      } else {
        res += v / 3 + 1;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 3, 3, 2, 4, 4, 4, 4, 4}, 4},
      {{2, 3, 3}, -1},
  };

  for (auto& [tasks, ans] : tests) {
    assert(Solution().minimumRounds(tasks) == ans);
  }
}
