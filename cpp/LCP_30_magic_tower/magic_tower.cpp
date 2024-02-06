/*
 * @Date: 2024-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-06
 * @FilePath: /algorithm/cpp/LCP_30_magic_tower/magic_tower.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int magicTower(vector<int>& nums) {
    priority_queue<int, vector<int>, greater<int>> q;
    int ans = 0;
    long long hp = 1, delay = 0;
    for (int num : nums) {
      if (num < 0) {
        q.push(num);
      }
      hp += num;
      if (hp <= 0) {
        ++ans;
        delay += q.top();
        hp -= q.top();
        q.pop();
      }
    }
    hp += delay;
    return hp <= 0 ? -1 : ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{100, 100, 100, -250, -60, -140, -50, -50, 100, 150}, 1},
      {{-200, -300, 400, 0}, -1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().magicTower(nums) == ans);
  }
}