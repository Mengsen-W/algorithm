/*
 * @Date: 2021-08-29 16:57:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-29 17:10:52
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumOddLengthSubarrays(vector<int>& arr) {
    int sum = 0;
    int n = arr.size();
    for (int i = 0; i < n; ++i) {
      int left_count = i;
      int right_count = n - i - 1;
      int left_odd = (left_count + 1) / 2;
      int right_odd = (right_count + 1) / 2;
      int left_even = left_count / 2 + 1;
      int right_even = right_count / 2 + 1;
      sum += arr[i] * (left_odd * right_odd + left_even * right_even);
    }
    return sum;
  }
};

int main() {
  {
    vector<int> arr{1, 4, 2, 5, 3};
    int ans = 58;
    assert(Solution().sumOddLengthSubarrays(arr) == ans);
  }
  {
    vector<int> arr{1, 2};
    int ans = 3;
    assert(Solution().sumOddLengthSubarrays(arr) == ans);
  }
  {
    vector<int> arr{10, 11, 12};
    int ans = 66;
    assert(Solution().sumOddLengthSubarrays(arr) == ans);
  }
}