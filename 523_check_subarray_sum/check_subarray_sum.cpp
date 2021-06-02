/*
 * @Date: 2021-06-02 07:45:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-02 07:50:40
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

bool checkSubarraySum(vector<int> nums, int k) {
  unordered_map<int, int> m = {{0, -1}};
  int rem = 0;
  for (int i = 0; i < nums.size(); ++i) {
    rem = (rem + nums[i]) % k;
    if (m.count(rem)) {
      int pos = m[rem];
      if ((i - pos) >= 2) return true;
    } else
      m[rem] = i;
  }
  return false;
}

int main() {
  assert(checkSubarraySum(vector<int>{23, 2, 4, 6, 7}, 6));
  assert(checkSubarraySum(vector<int>{23, 2, 6, 4, 7}, 6));
  assert(!checkSubarraySum(vector<int>{23, 2, 6, 4, 7}, 13));
}