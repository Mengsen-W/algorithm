/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/cpp/1043_max_sum_after_partitioning/max_sum_after_partitioning.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  int maxSumAfterPartitioning(vector<int>& arr, int k) {
    int n = arr.size();
    vector<int> d(n + 1);
    for (int i = 1; i <= n; i++) {
      int maxValue = arr[i - 1];
      for (int j = i - 1; j >= 0 && j >= i - k; j--) {
        d[i] = max(d[i], d[j] + maxValue * (i - j));
        if (j > 0) {
          maxValue = max(maxValue, arr[j - 1]);
        }
      }
    }
    return d[n];
  }
};

int main() {
  {
    vector<int> arr{1, 15, 7, 9, 2, 5, 10};
    int k = 3;
    int ans = 84;
    assert(Solution().maxSumAfterPartitioning(arr, k) == ans);
  }

  {
    vector<int> arr{1,4,1,5,7,3,6,1,9,9,3};
    int k = 4;
    int ans = 83;
    assert(Solution().maxSumAfterPartitioning(arr, k) == ans);
  }

  {
    vector<int> arr{1,};
    int k = 1;
    int ans = 1;
    assert(Solution().maxSumAfterPartitioning(arr, k) == ans);
  }
}