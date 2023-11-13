/*
 * @Date: 2022-04-03 23:23:04
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-13
 * @FilePath: /algorithm/cpp/307_NumArray/NumArray.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class NumArray {
 private:
  vector<int> tree;
  vector<int>& nums;

  int lowBit(int x) { return x & -x; }

  void add(int index, int val) {
    int tree_size = tree.size();
    while (index < tree_size) {
      tree[index] += val;
      index += lowBit(index);
    }
  }

  int prefixSum(int index) {
    int sum = 0;
    while (index > 0) {
      sum += tree[index];
      index -= lowBit(index);
    }
    return sum;
  }

 public:
  NumArray(vector<int>& nums) : tree(nums.size() + 1), nums(nums) {
    int nums_size = nums.size();
    for (int i = 0; i < nums_size; i++) {
      add(i + 1, nums[i]);
    }
  }

  void update(int index, int val) {
    add(index + 1, val - nums[index]);
    nums[index] = val;
  }

  int sumRange(int left, int right) { return prefixSum(right + 1) - prefixSum(left); }
};

int main() {
  vector<int> nums{1, 3, 5};
  NumArray n{nums};
  assert(n.sumRange(0, 2) == 9);
  n.update(1, 2);
  assert(n.sumRange(0, 2) == 8);
}