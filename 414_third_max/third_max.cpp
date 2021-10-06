/*
 * @Date: 2021-10-06 10:12:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-06 10:23:50
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int thirdMax(vector<int> &nums) {
    int *a = nullptr, *b = nullptr, *c = nullptr;
    for (int &num : nums) {
      if (a == nullptr || num > *a) {
        c = b;
        b = a;
        a = &num;
      } else if (*a > num && (b == nullptr || num > *b)) {
        c = b;
        b = &num;
      } else if (b != nullptr && *b > num && (c == nullptr || num > *c)) {
        c = &num;
      }
    }
    return c == nullptr ? *a : *c;
  }
};

int main() {
  {
    vector<int> nums{3, 2, 1};
    int ans = 1;
    assert(Solution().thirdMax(nums) == ans);
  }
  {
    vector<int> nums{2, 1};
    int ans = 2;
    assert(Solution().thirdMax(nums) == ans);
  }
  {
    vector<int> nums{2, 3, 3, 1};
    int ans = 1;
    assert(Solution().thirdMax(nums) == ans);
  }
}