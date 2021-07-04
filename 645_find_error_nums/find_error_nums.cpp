/*
 * @Date: 2021-07-04 10:22:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-04 12:29:44
 */

#include <cassert>
#include <vector>
using namespace std;

vector<int> findErrorNums(vector<int>& nums) {
  int n = nums.size();
  int xorSum = 0;
  for (int num : nums) xorSum ^= num;

  for (int i = 1; i <= n; i++) xorSum ^= i;

  int lowbit = xorSum & (-xorSum);
  int num1 = 0, num2 = 0;
  for (int& num : nums)
    if ((num & lowbit) == 0)
      num1 ^= num;
    else
      num2 ^= num;

  for (int i = 1; i <= n; i++)
    if ((i & lowbit) == 0)
      num1 ^= i;
    else
      num2 ^= i;

  for (int num : nums)
    if (num == num1) return vector<int>{num1, num2};

  return vector<int>{num2, num1};
}

int main() {
  {
    vector<int> nums{1, 2, 2, 4};
    assert(findErrorNums(nums) == std::move(vector<int>{2, 3}));
  }
  {
    vector<int> nums{1, 1};
    assert(findErrorNums(nums) == std::move(vector<int>{1, 2}));
  }
}