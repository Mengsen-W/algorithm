/*
 * @Date: 2021-12-02 06:06:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-02 06:08:28
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> findRelativeRanks(vector<int> score) {
    int n = score.size();
    string desc[3] = {"Gold Medal", "Silver Medal", "Bronze Medal"};
    vector<pair<int, int>> arr;

    for (int i = 0; i < n; ++i) arr.emplace_back(make_pair(-score[i], i));

    sort(arr.begin(), arr.end());
    vector<string> ans(n);
    for (int i = 0; i < n; ++i)
      if (i >= 3)
        ans[arr[i].second] = to_string(i + 1);
      else
        ans[arr[i].second] = desc[i];

    return ans;
  }
};

int main() {
  assert(
      (Solution().findRelativeRanks({5, 4, 3, 2, 1}) ==
       vector<string>{"Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"}));
  assert(
      (Solution().findRelativeRanks({10, 3, 8, 9, 4}) ==
       vector<string>{"Gold Medal", "5", "Bronze Medal", "Silver Medal", "6"}));
}