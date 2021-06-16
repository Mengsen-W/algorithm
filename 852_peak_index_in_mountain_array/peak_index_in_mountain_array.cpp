/*
 * @Date: 2021-06-15 08:40:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-15 08:44:33
 */

#include <cassert>
#include <vector>
using namespace std;

int peakIndexInMountainArray(vector<int>& arr) {
  int n = arr.size();
  int left = 1, right = n - 2, ans = 0;
  while (left <= right) {
    int mid = (left + right) / 2;
    if (arr[mid] > arr[mid + 1]) {
      ans = mid;
      right = mid - 1;
    } else {
      left = mid + 1;
    }
  }
  return ans;
}

int main() {
  {
    vector<int> arr{0, 1, 0};
    int ans = 1;
    assert(peakIndexInMountainArray(arr) == ans);
  }
  {
    vector<int> arr{0, 2, 1, 0};
    int ans = 1;
    assert(peakIndexInMountainArray(arr) == ans);
  }
  {
    vector<int> arr{0, 10, 5, 2, 0};
    int ans = 1;
    assert(peakIndexInMountainArray(arr) == ans);
  }
  {
    vector<int> arr{3, 4, 5, 1};
    int ans = 2;
    assert(peakIndexInMountainArray(arr) == ans);
  }
  {
    vector<int> arr{24, 69, 100, 99, 79, 78, 67, 36, 26, 19};
    int ans = 2;
    assert(peakIndexInMountainArray(arr) == ans);
  }
}