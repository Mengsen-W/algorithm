/*
 * @Date: 2023-09-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-02
 * @FilePath: /algorithm/cpp/2511_capture_forts/capture_forts.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int captureForts(vector<int>& forts) {
    int ans = 0, pre = -1;
    for (int i = 0; i < forts.size(); i++) {
      if (forts[i] == 1 || forts[i] == -1) {
        if (pre >= 0 && forts[i] != forts[pre]) {
          ans = max(ans, i - pre - 1);
        }
        pre = i;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 0, 0, -1, 0, 0, 0, 0, 1}, 4},
      {{0, 0, 1, -1}, 0},
  };

  for (auto& [forts, ans] : tests) {
    assert(Solution().captureForts(forts) == ans);
  }
}