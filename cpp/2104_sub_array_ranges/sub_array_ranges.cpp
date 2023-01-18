/*
 * @Date: 2022-03-04 00:51:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-04 01:47:57
 * @FilePath: /algorithm/2104_sub_array_ranges/sub_array_ranges.cpp
 */

#include <cassert>
#include <climits>
#include <stack>
#include <vector>

using namespace std;

class Solution1 {
 public:
  long long subArrayRanges(vector<int> nums) {
    int n = nums.size();
    long long ret = 0;
    for (int i = 0; i < n; i++) {
      int minVal = INT_MAX, maxVal = INT_MIN;
      for (int j = i; j < n; j++) {
        minVal = min(minVal, nums[j]);
        maxVal = max(maxVal, nums[j]);
        ret += maxVal - minVal;
      }
    }
    return ret;
  }
};

class Solution2 {
 public:
  long long subArrayRanges(vector<int> nums) {
    int n = nums.size();
    vector<int> minLeft(n), minRight(n), maxLeft(n), maxRight(n);
    stack<int> minStack, maxStack;
    for (int i = 0; i < n; i++) {
      while (!minStack.empty() && nums[minStack.top()] > nums[i]) {
        minStack.pop();
      }
      minLeft[i] = minStack.empty() ? -1 : minStack.top();
      minStack.push(i);

      // 如果 nums[maxStack.top()] == nums[i], 那么根据定义，
      // nums[maxStack.top()] 逻辑上小于 nums[i]，因为 maxStack.top() < i
      while (!maxStack.empty() && nums[maxStack.top()] <= nums[i]) {
        maxStack.pop();
      }
      maxLeft[i] = maxStack.empty() ? -1 : maxStack.top();
      maxStack.push(i);
    }
    minStack = stack<int>();
    maxStack = stack<int>();
    for (int i = n - 1; i >= 0; i--) {
      // 如果 nums[minStack.top()] == nums[i], 那么根据定义，
      // nums[minStack.top()] 逻辑上大于 nums[i]，因为 minStack.top() > i
      while (!minStack.empty() && nums[minStack.top()] >= nums[i]) {
        minStack.pop();
      }
      minRight[i] = minStack.empty() ? n : minStack.top();
      minStack.push(i);

      while (!maxStack.empty() && nums[maxStack.top()] < nums[i]) {
        maxStack.pop();
      }
      maxRight[i] = maxStack.empty() ? n : maxStack.top();
      maxStack.push(i);
    }

    long long sumMax = 0, sumMin = 0;
    for (int i = 0; i < n; i++) {
      sumMax +=
          static_cast<long long>(maxRight[i] - i) * (i - maxLeft[i]) * nums[i];
      sumMin +=
          static_cast<long long>(minRight[i] - i) * (i - minLeft[i]) * nums[i];
    }
    return sumMax - sumMin;
  }
};

int main() {
  assert(Solution1().subArrayRanges({1, 2, 3}) == 4);
  assert(Solution2().subArrayRanges({1, 2, 3}) == 4);

  assert(Solution1().subArrayRanges({1, 3, 3}) == 4);
  assert(Solution2().subArrayRanges({1, 3, 3}) == 4);

  assert(Solution1().subArrayRanges({4, -2, -3, 4, 1}) == 59);
  assert(Solution2().subArrayRanges({4, -2, -3, 4, 1}) == 59);
}