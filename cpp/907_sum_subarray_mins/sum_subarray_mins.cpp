/*
 * @Date: 2022-10-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-27
 * @FilePath: /algorithm/cpp/907_sum_subarray_mins/sum_subarray_mins.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumSubarrayMins(vector<int>& arr) {
    int n = arr.size();
    vector<int> monoStack;
    vector<int> left(n), right(n);
    for (int i = 0; i < n; i++) {
      while (!monoStack.empty() && arr[i] <= arr[monoStack.back()]) {
        monoStack.pop_back();
      }
      left[i] = i - (monoStack.empty() ? -1 : monoStack.back());
      monoStack.emplace_back(i);
    }
    monoStack.clear();
    for (int i = n - 1; i >= 0; i--) {
      while (!monoStack.empty() && arr[i] < arr[monoStack.back()]) {
        monoStack.pop_back();
      }
      right[i] = (monoStack.empty() ? n : monoStack.back()) - i;
      monoStack.emplace_back(i);
    }
    long long ans = 0;
    long long mod = 1e9 + 7;
    for (int i = 0; i < n; i++) {
      ans = (ans + (long long)left[i] * right[i] * arr[i]) % mod;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 1, 2, 4}, 17},
      {{11, 81, 94, 43, 3}, 444},
  };

  for (auto &[arr, ans] : tests) {
    assert(Solution().sumSubarrayMins(arr) == ans);
  }
}