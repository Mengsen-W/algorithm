/*
 * @Date: 2022-03-06 00:57:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-06 02:08:57
 * @FilePath: /algorithm/2100_good_days_to_rob_bank/good_days_to_rob_bank.cpp
 * @Description: file content
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> goodDaysToRobBank(vector<int> security, int time) {
    int n = security.size();
    vector<int> left(n);
    vector<int> right(n);
    for (int i = 1; i < n; i++) {
      if (security[i] <= security[i - 1]) {
        left[i] = left[i - 1] + 1;
      }
      if (security[n - i - 1] <= security[n - i]) {
        right[n - i - 1] = right[n - i] + 1;
      }
    }

    vector<int> ans;
    for (int i = time; i < n - time; i++) {
      if (left[i] >= time && right[i] >= time) {
        ans.emplace_back(i);
      }
    }
    return ans;
  }
};

int main() {
  assert((Solution().goodDaysToRobBank({5, 3, 3, 3, 5, 6, 2}, 2) ==
          vector<int>{2, 3}));
  assert((Solution().goodDaysToRobBank({1, 1, 1, 1, 1}, 0) ==
          vector<int>{0, 1, 2, 3, 4}));
  assert(Solution().goodDaysToRobBank({1, 2, 3, 4, 5, 6}, 2) == vector<int>{});
  assert(Solution().goodDaysToRobBank({1}, 5) == vector<int>{});
}