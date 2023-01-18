/*
 * @Date: 2021-10-14 08:47:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-14 08:54:07
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int _peakIndexInMountainArray(vector<int> arr) {
    return partition_point(arr.begin() + 1, arr.end() - 1,
                           [](auto&& x) { return *((&x) + 1) > x; }) -
           arr.begin();
  }
  int peakIndexInMountainArray(vector<int> arr) {
    int l = 1, r = arr.size() - 2;
    while (l < r) {
      int m = (l + r) >> 1;
      arr[m] < arr[m + 1] ? l = m + 1 : r = m;
    }
    return l;
  }
};

int main() {
  assert(Solution().peakIndexInMountainArray(vector<int>{0, 1, 0}) == 1);
  assert(Solution().peakIndexInMountainArray(vector<int>{1, 3, 5, 4, 2}) == 2);
  assert(Solution().peakIndexInMountainArray(vector<int>{0, 10, 5, 2}) == 1);
  assert(Solution().peakIndexInMountainArray(vector<int>{3, 4, 5, 1}) == 2);
  assert(Solution().peakIndexInMountainArray(
             vector<int>{24, 69, 100, 99, 79, 78, 67, 36, 26, 19}) == 2);
  assert(Solution()._peakIndexInMountainArray(vector<int>{0, 1, 0}) == 1);
  assert(Solution()._peakIndexInMountainArray(vector<int>{1, 3, 5, 4, 2}) == 2);
  assert(Solution()._peakIndexInMountainArray(vector<int>{0, 10, 5, 2}) == 1);
  assert(Solution()._peakIndexInMountainArray(vector<int>{3, 4, 5, 1}) == 2);
  assert(Solution()._peakIndexInMountainArray(
             vector<int>{24, 69, 100, 99, 79, 78, 67, 36, 26, 19}) == 2);
}
