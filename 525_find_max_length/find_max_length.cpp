/*
 * @Date: 2021-06-03 08:19:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-03 08:36:29
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

int findMaxLength(vector<int> nums) {
  unordered_map<int, int> map;
  int sum = 0, ans = 0;

  map[0] = -1;
  for (int i = 0; i < nums.size(); i++) {
    sum += nums[i] ? 1 : -1;
    auto p = map.find(sum);
    if (p != map.end())
      ans = max(ans, i - p->second);
    else
      map[sum] = i;
  }
  return ans;
}

int main() {
  assert(findMaxLength(vector<int>{0, 1}) == 2);
  assert(findMaxLength(vector<int>{0, 1, 0}) == 2);
}