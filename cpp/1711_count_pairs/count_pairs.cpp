/*
 * @Date: 2021-07-07 08:54:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-07 08:58:15
 */

#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

static constexpr int MOD = 1'000'000'007;

int countPairs(vector<int>& deliciousness) {
  int maxVal = *max_element(deliciousness.begin(), deliciousness.end());
  int maxSum = maxVal * 2;
  int pairs = 0;
  unordered_map<int, int> mp;
  int n = deliciousness.size();
  for (auto& val : deliciousness) {
    for (int sum = 1; sum <= maxSum; sum <<= 1) {
      int count = mp.count(sum - val) ? mp[sum - val] : 0;
      pairs = (pairs + count) % MOD;
    }
    mp[val]++;
  }
  return pairs;
}

int main() {
  {
    vector<int> deliciousness{1, 3, 5, 7, 9};
    assert(countPairs(deliciousness) == 4);
  }
  {
    vector<int> deliciousness{1, 1, 1, 3, 3, 3, 7};
    assert(countPairs(deliciousness) == 15);
  }
}