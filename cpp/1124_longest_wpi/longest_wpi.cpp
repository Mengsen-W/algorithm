/*
 * @Date: 2023-02-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-14
 * @FilePath: /algorithm/cpp/1124_longest_wpi/longest_wpi.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestWPI(vector<int>& hours) {
    int n = hours.size();
    unordered_map<int, int> ump;
    int s = 0, res = 0;
    for (int i = 0; i < n; i++) {
      s += hours[i] > 8 ? 1 : -1;
      if (s > 0) {
        res = max(res, i + 1);
      } else {
        if (ump.count(s - 1)) {
          res = max(res, i - ump[s - 1]);
        }
      }
      if (!ump.count(s)) {
        ump[s] = i;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> hours{9, 9, 6, 0, 6, 6, 9};
    int ans = 3;
    assert(Solution().longestWPI(hours) == ans);
  }

  {
    vector<int> hours{6,6,6};
    int ans = 0;
    assert(Solution().longestWPI(hours) == ans);
  }
}
