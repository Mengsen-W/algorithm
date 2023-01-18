/*
 * @Date: 2021-12-27 01:43:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-27 02:13:31
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numFriendRequests(vector<int> ages) {
    vector<int> cnt(121);
    for (int age : ages) {
      ++cnt[age];
    }
    vector<int> pre(121);
    for (int i = 1; i <= 120; ++i) {
      pre[i] = pre[i - 1] + cnt[i];
    }
    int ans = 0;
    for (int i = 15; i <= 120; ++i) {
      if (cnt[i]) {
        int bound = i * 0.5 + 8;
        ans += cnt[i] * (pre[i] - pre[bound - 1] - 1);
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().numFriendRequests({16, 16}) == 2);
  assert(Solution().numFriendRequests({16, 17, 18}) == 2);
  assert(Solution().numFriendRequests({20, 30, 100, 110, 120}) == 3);
}