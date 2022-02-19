/*
 * @Date: 2022-02-19 14:08:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-19 14:14:33
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> pancakeSort(vector<int> arr) {
    vector<int> ret;
    for (int n = arr.size(); n > 1; n--) {
      int index = max_element(arr.begin(), arr.begin() + n) - arr.begin();
      if (index == n - 1) {
        continue;
      }
      reverse(arr.begin(), arr.begin() + index + 1);
      reverse(arr.begin(), arr.begin() + n);
      ret.push_back(index + 1);
      ret.push_back(n);
    }
    return ret;
  }
};

int main() {
  assert(
      (Solution().pancakeSort({3, 2, 4, 1}) == vector<int>{3, 4, 2, 3, 1, 2}));
  assert((Solution().pancakeSort({3, 2, 1}) == vector<int>{1, 3}));
}