/*
 * @Date: 2023-05-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-07
 * @FilePath: /algorithm/cpp/1010_num_pairs_divisible_by60/num_pairs_divisible_by60.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numPairsDivisibleBy60(vector<int>& time) {
    vector<int> cnt(60);
    for (int t : time) {
      cnt[t % 60]++;
    }
    long long res = 0;
    for (int i = 1; i < 30; i++) {
      res += cnt[i] * cnt[60 - i];
    }
    res += (long long)cnt[0] * (cnt[0] - 1) / 2 + (long long)cnt[30] * (cnt[30] - 1) / 2;
    return (int)res;
  }
};

int main() {
  {
    vector<int> time{30, 20, 150, 100, 40};
    int ans = 3;
    assert(Solution().numPairsDivisibleBy60(time) == ans);
  }

  {
    vector<int> time{60, 60, 60};
    int ans = 3;
    assert(Solution().numPairsDivisibleBy60(time) == ans);
  }
}