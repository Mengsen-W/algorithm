/*
 * @Date: 2021-04-04 18:39:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:47:34
 * @FilePath: \algorithm\90_subsets_with_dup\subsets_with_dup.cpp
 * @Description: file content
 */

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

vector<int> t;
vector<vector<int>> ans;

void dfs(bool choosePre, int cur, vector<int> &nums) {
  if (cur == nums.size()) {
    ans.push_back(t);
    return;
  }
  dfs(false, cur + 1, nums);
  if (!choosePre && cur > 0 && nums[cur - 1] == nums[cur]) {
    return;
  }
  t.push_back(nums[cur]);
  dfs(true, cur + 1, nums);
  t.pop_back();
}

vector<vector<int>> subsets_with_dup(vector<int> &nums) {
  sort(nums.begin(), nums.end());
  dfs(false, 0, nums);
  return ans;
}

int main() {
  vector<int> nums{1, 2, 2};
  subsets_with_dup(nums);
  for (auto v : ans) {
    for (int i : v) {
      cout << i << ", ";
    }
    cout << endl;
  }

  ans.clear();
  nums.clear();
  nums = {0};
  subsets_with_dup(nums);
  for (auto v : ans) {
    for (int i : v) {
      cout << i << ", ";
    }
    cout << endl;
  }
}