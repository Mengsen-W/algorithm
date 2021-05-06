/*
 * @Date: 2021-05-06 10:51:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-06 10:56:43
 */

#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

int rob(vector<int> &nums) {
  int size = nums.size();
  if (size == 1) {
    return nums[0];
  }
  int first = nums[0], second = max(nums[0], nums[1]);
  for (int i = 2; i < size; i++) {
    int temp = second;
    second = max(first + nums[i], second);
    first = temp;
  }
  return second;
}

int delete_and_earn(vector<int> &nums) {
  int n = nums.size();
  int ans = 0;
  sort(nums.begin(), nums.end());
  vector<int> sum = {nums[0]};
  for (int i = 1; i < n; ++i) {
    int val = nums[i];
    if (val == nums[i - 1]) {
      sum.back() += val;
    } else if (val == nums[i - 1] + 1) {
      sum.push_back(val);
    } else {
      ans += rob(sum);
      sum = {val};
    }
  }
  ans += rob(sum);
  return ans;
}

int main() {
  {
    vector<int> nums{3, 4, 2};
    assert(delete_and_earn(nums) == 6);
  }
  {
    vector<int> nums{2, 2, 3, 3, 3, 4};
    assert(delete_and_earn(nums) == 9);
  }
  return 0;
}