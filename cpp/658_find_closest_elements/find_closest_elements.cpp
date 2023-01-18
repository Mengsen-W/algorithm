/*
 * @Date: 2022-08-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-25
 * @FilePath: /algorithm/658_find_closest_elements/find_closest_elements.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findClosestElements(vector<int>& arr, int k, int x) {
    int right = lower_bound(arr.begin(), arr.end(), x) - arr.begin();
    int left = right - 1;
    while (k--) {
      if (left < 0)
        right++;
      else if (right >= arr.size())
        left--;
      else if (x - arr[left] <= arr[right] - x)
        left--;
      else
        right++;
    }
    return vector<int>(arr.begin() + left + 1, arr.begin() + right);
  }
};

int main() {
  {
    vector<int> arr{1, 2, 3, 4, 5};
    int k = 4;
    int x = 3;
    vector<int> ans{1, 2, 3, 4};
    assert(Solution().findClosestElements(arr, k, x) == ans);
  }

  {
    vector<int> arr{1, 2, 3, 4, 5};
    int k = 4;
    int x = -1;
    vector<int> ans{1, 2, 3, 4};
    assert(Solution().findClosestElements(arr, k, x) == ans);
  }
}