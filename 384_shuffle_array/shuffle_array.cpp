/*
 * @Date: 2021-11-22 02:07:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-22 02:18:43
 */

#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  Solution(vector<int>& nums) {
    this->nums = nums;
    this->original.resize(nums.size());
    copy(nums.begin(), nums.end(), original.begin());
  }

  vector<int> reset() {
    copy(original.begin(), original.end(), nums.begin());
    return nums;
  }

  vector<int> shuffle() {
    int nums_size = nums.size();
    for (int i = 0; i < nums_size; ++i) {
      int j = i + rand() % (nums.size() - i);
      swap(nums[i], nums[j]);
    }
    return nums;
  }

 private:
  vector<int> nums;
  vector<int> original;
};

void print_vector(vector<int> nums) {
    int nums_size = nums.size();
  for (int i = 0; i < nums_size; ++i) {
    cout << nums[i] << " ";
  }
  cout << endl;
}

int main() {
  vector<int> nums = {1, 2, 3};
  Solution s(nums);
  print_vector(s.shuffle());
  print_vector(s.reset());
  print_vector(s.shuffle());
}