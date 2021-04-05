/*
 * @Date: 2021-04-05 10:26:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-05 10:31:16
 * @FilePath: \algorithm\83_merge\merge.cpp
 * @Description: file content
 */

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
  int p1 = m - 1, p2 = n - 1;
  int tail = m + n - 1;
  int cur;
  while (p1 >= 0 || p2 >= 0) {
    if (p1 == -1) {
      cur = nums2[p2--];
    } else if (p2 == -1) {
      cur = nums1[p1--];
    } else if (nums1[p1] > nums2[p2]) {
      cur = nums1[p1--];
    } else {
      cur = nums2[p2--];
    }
    nums1[tail--] = cur;
  }
}

int main() {
  {
    vector<int> nums1 = {1, 2, 3, 0, 0, 0};
    int m = 3;
    vector<int> nums2 = {2, 5, 6};
    int n = 3;
    merge(nums1, m, nums2, n);
    for (int i : nums1) cout << i << ", ";
  }
  cout << endl;
  {
    vector<int> nums1 = {1};
    int m = 1;
    vector<int> nums2 = {};
    int n = 0;
    merge(nums1, m, nums2, n);
    for (int i : nums1) cout << i << ", ";
  }
}