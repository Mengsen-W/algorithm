/*
 * @Date: 2024-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-01
 * @FilePath: /algorithm/cpp/lcp_24_nums_game/nums_game.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numsGame(vector<int>& nums) {
    int n = nums.size();
    vector<int> res(n);
    priority_queue<int> lower;
    priority_queue<int, vector<int>, greater<>> upper;
    long long mod = 1e9 + 7;
    long long lowerSum = 0, upperSum = 0;
    for (int i = 0; i < n; i++) {
      int x = nums[i] - i;
      if (lower.empty() || lower.top() >= x) {
        lowerSum += x;
        lower.push(x);
        if (lower.size() > upper.size() + 1) {
          upperSum += lower.top();
          upper.push(lower.top());
          lowerSum -= lower.top();
          lower.pop();
        }
      } else {
        upperSum += x;
        upper.push(x);
        if (lower.size() < upper.size()) {
          lowerSum += upper.top();
          lower.push(upper.top());
          upperSum -= upper.top();
          upper.pop();
        }
      }
      if ((i + 1) % 2 == 0) {
        res[i] = (upperSum - lowerSum) % mod;
      } else {
        res[i] = (upperSum - lowerSum + lower.top()) % mod;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{3, 4, 5, 1, 6, 7}, {0, 0, 0, 5, 6, 7}},
      {{1, 2, 3, 4, 5}, {0, 0, 0, 0, 0}},
      {{1, 1, 1, 2, 3, 4}, {0, 1, 2, 3, 3, 3}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().numsGame(nums) == ans);
  }
}
