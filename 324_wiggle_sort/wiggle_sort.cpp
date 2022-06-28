/*
 * @Date: 2022-06-28
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-28
 * @FilePath: /algorithm/324_wiggle_sort/wiggle_sort.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  void wiggleSort(vector<int>& nums) {
    int n = nums.size();
    int x = (n + 1) / 2;
    int mid = x - 1;
    nth_element(nums.begin(), nums.begin() + mid, nums.end());
    for (int k = 0, i = 0, j = n - 1; k <= j; k++) {
      if (nums[k] > nums[mid]) {
        while (j > k && nums[j] > nums[mid]) {
          j--;
        }
        swap(nums[k], nums[j--]);
      }
      if (nums[k] < nums[mid]) {
        swap(nums[k], nums[i++]);
      }
    }
    vector<int> arr = nums;
    for (int i = 0, j = x - 1, k = n - 1; i < n; i += 2, j--, k--) {
      nums[i] = arr[j];
      if (i + 1 < n) {
        nums[i + 1] = arr[k];
      }
    }
  }
};

int main() {
  {
    vector<int> nums{1, 5, 1, 1, 6, 4};
    vector<int> ans{1, 6, 1, 5, 1, 4};
    Solution().wiggleSort(nums);
    assert(nums == ans);
  }

  {
    vector<int> nums{1, 3, 2, 2, 3, 1};
    vector<int> ans{2, 3, 1, 3, 1, 2};
    Solution().wiggleSort(nums);
    assert(nums == ans);
  }
}