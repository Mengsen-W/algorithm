/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-20 09:07:09
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-20 09:17:11
 */

#include <cassert>
#include <vector>

using namespace std;

int find_shortest_subarray(vector<int>& nums) {
  //统计每个元素出现的频率
  vector<int> req(50000, 0);
  //最大度
  int d = 0;
  //每个元素第一次出现的位置
  vector<int> first(50000, -1);
  //每个元素最后一次出现的位置
  vector<int> last(50000, -1);
  int res = nums.size();
  for (int i = 0; i < nums.size(); i++) {
    req[nums[i]]++;
    d = max(d, req[nums[i]]);
    if (first[nums[i]] == -1) first[nums[i]] = i;
    last[nums[i]] = i;
  }
  for (int i = 0; i < nums.size(); i++) {
    if (req[nums[i]] == d) {
      res = min(res, last[nums[i]] - first[nums[i]] + 1);
    }
  }
  return res;
}

int main() {
  vector<int> nums{};
  nums = {1, 2, 2, 3, 1};
  assert(find_shortest_subarray(nums) == 2);
  nums = {1, 2, 2, 3, 1};
  assert(find_shortest_subarray(nums) == 2);
  nums = {1, 2, 2, 3};
  assert(find_shortest_subarray(nums) == 2);
  nums = {2, 2, 3, 1};
  assert(find_shortest_subarray(nums) == 2);
  nums = {1, 2, 2};
  assert(find_shortest_subarray(nums) == 2);
  nums = {2, 2, 3};
  assert(find_shortest_subarray(nums) == 2);
  nums = {2, 2};
  assert(find_shortest_subarray(nums) == 2);
  nums = {1, 2, 2, 3, 1, 4, 2};
  assert(find_shortest_subarray(nums) == 6);
}