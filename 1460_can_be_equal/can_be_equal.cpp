/*
 * @Date: 2022-08-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-24
 * @FilePath: /algorithm/1460_can_be_equal/can_be_equal.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canBeEqual(vector<int>& target, vector<int>& arr) {
    sort(target.begin(), target.end());
    sort(arr.begin(), arr.end());
    return target == arr;
  }
};

int main() {
  {
    vector<int> target{1, 2, 3, 4};
    vector<int> arr{2, 4, 1, 3};
    assert(Solution().canBeEqual(target, arr));
  }
  {
    vector<int> target{7};
    vector<int> arr{7};
    assert(Solution().canBeEqual(target, arr));
  }

  {
    vector<int> target{3, 7, 9};
    vector<int> arr{3, 7, 11};
    assert(!Solution().canBeEqual(target, arr));
  }
}