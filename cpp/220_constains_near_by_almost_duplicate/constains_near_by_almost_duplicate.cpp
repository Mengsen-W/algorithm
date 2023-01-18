/*
 * @Date: 2021-04-17 10:10:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-17 11:50:57
 */

#include <cassert>
#include <iostream>
#include <set>
#include <unordered_map>
#include <vector>
using namespace std;

int get_ID(int x, long w) { return x < 0 ? (x + 1ll) / w - 1 : x / w; }

bool contains_near_by_almost_duplicate(vector<int>& nums, int k, int t) {
  unordered_map<int, int> mp;
  int n = nums.size();
  for (int i = 0; i < n; i++) {
    long x = nums[i];
    int id = get_ID(x, t + 1ll);
    if (mp.count(id)) {
      return true;
    }
    if (mp.count(id - 1) && abs(x - mp[id - 1]) <= t) {
      return true;
    }
    if (mp.count(id + 1) && abs(x - mp[id + 1]) <= t) {
      return true;
    }
    mp[id] = x;
    if (i >= k) {
      mp.erase(get_ID(nums[i - k], t + 1ll));
    }
  }
  return false;
}

bool contains_near_by_almost_duplicate_2(vector<int>& nums, int k, int t) {
  int n = nums.size();
  set<int> rec;
  for (int i = 0; i < n; i++) {
    auto iter = rec.lower_bound(max(nums[i], INT_MIN + t) - t);
    if (iter != rec.end() && *iter <= min(nums[i], INT_MAX - t) + t) {
      return true;
    }
    rec.insert(nums[i]);
    if (i >= k) {
      rec.erase(nums[i - k]);
    }
  }
  return false;
}

int main() {
  {
    vector<int> nums{1, 2, 3, 1};
    assert(contains_near_by_almost_duplicate(nums, 3, 0));
  }
  {
    vector<int> nums{1, 0, 1, 1};
    assert(contains_near_by_almost_duplicate(nums, 1, 2));
  }
  {
    vector<int> nums{1, 5, 9, 1, 5, 9};
    assert(!contains_near_by_almost_duplicate(nums, 2, 3));
  }
  {
    vector<int> nums{2147483647, -1, 2147483647};
    assert(!contains_near_by_almost_duplicate_2(nums, 1, 2147483647));
  }
  return 0;
}