/*
 * @Date: 2023-09-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-30
 * @FilePath: /algorithm/cpp/2136_earliest_full_bloom/earliest_full_bloom.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int earliestFullBloom(vector<int>& plantTime, vector<int>& growTime) {
    int n = plantTime.size();
    vector<int> id(n);
    iota(id.begin(), id.end(), 0);
    sort(id.begin(), id.end(), [&](int i, int j) { return growTime[i] > growTime[j]; });
    int prev = 0, ans = 0;
    for (int i : id) {
      ans = max(ans, prev + plantTime[i] + growTime[i]);
      prev += plantTime[i];
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 4, 3}, {2, 3, 1}, 9},
      {{1,2,3,2}, {2,1,2,1}, 9},
      {{1},{1},2},
  };

  for (auto &[plantTime, growTime, expected] : tests) {
    assert(Solution().earliestFullBloom(plantTime, growTime) == expected);
  }
}