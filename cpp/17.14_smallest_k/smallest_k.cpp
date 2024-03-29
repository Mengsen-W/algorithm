/*
 * @Date: 2021-09-03 13:22:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-03 13:27:52
 */

#include <cassert>
#include <ctime>
#include <random>
#include <vector>

using namespace std;

class Solution {
  int partition(vector<int>& nums, int l, int r) {
    int pivot = nums[r];
    int i = l - 1;
    for (int j = l; j <= r - 1; ++j) {
      if (nums[j] <= pivot) {
        i = i + 1;
        swap(nums[i], nums[j]);
      }
    }
    swap(nums[i + 1], nums[r]);
    return i + 1;
  }

  // 基于随机的划分
  int randomized_partition(vector<int>& nums, int l, int r) {
    int i = rand() % (r - l + 1) + l;
    swap(nums[r], nums[i]);
    return partition(nums, l, r);
  }

  void randomized_selected(vector<int>& arr, int l, int r, int k) {
    if (l >= r) {
      return;
    }
    int pos = randomized_partition(arr, l, r);
    int num = pos - l + 1;
    if (k == num) {
      return;
    } else if (k < num) {
      randomized_selected(arr, l, pos - 1, k);
    } else {
      randomized_selected(arr, pos + 1, r, k - num);
    }
  }

 public:
  vector<int> smallestK(vector<int>& arr, int k) {
    srand((unsigned)time(NULL));
    randomized_selected(arr, 0, (int)arr.size() - 1, k);
    vector<int> vec;
    for (int i = 0; i < k; ++i) {
      vec.push_back(arr[i]);
    }
    return vec;
  }
};

int main() {
  vector<int> arr{1, 3, 5, 7, 2, 4, 6, 8};
  int k = 4;
  vector<int> ans{1, 2, 3, 4};
  assert(Solution{}.smallestK(arr, k) == ans);
}