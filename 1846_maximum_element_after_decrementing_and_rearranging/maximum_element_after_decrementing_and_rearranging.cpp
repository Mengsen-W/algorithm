/*
 * @Date: 2021-07-15 09:17:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-15 09:23:25
 */

#include <cassert>
#include <vector>
using namespace std;

int maximumElementAfterDecrementingAndRearranging(vector<int> &arr) {
  int n = arr.size();
  vector<int> cnt(n + 1);
  for (int v : arr) ++cnt[min(v, n)];

  int miss = 0;
  for (int i = 1; i <= n; ++i)
    if (cnt[i] == 0)
      ++miss;
    else
      miss -= min(cnt[i] - 1, miss);  // miss 不会小于 0，故至多减去 miss 个元素

  return n - miss;
}

int main() {
  {
    vector<int> arr{2, 2, 1, 2, 1};
    int ans = 2;
    assert(maximumElementAfterDecrementingAndRearranging(arr) == ans);
  }
  {
    vector<int> arr{100, 1, 1000};
    int ans = 3;
    assert(maximumElementAfterDecrementingAndRearranging(arr) == ans);
  }
  {
    vector<int> arr{1, 2, 3, 4, 5};
    int ans = 5;
    assert(maximumElementAfterDecrementingAndRearranging(arr) == ans);
  }
}