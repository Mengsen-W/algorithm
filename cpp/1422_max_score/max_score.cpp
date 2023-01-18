/*
 * @Date: 2022-08-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-14
 * @FilePath: /algorithm/1422_max_score/max_score.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int maxScore(string s) {
    int score = 0;
    int n = s.size();
    if (s[0] == '0') {
      score++;
    }
    for (int i = 1; i < n; i++) {
      if (s[i] == '1') {
        score++;
      }
    }
    int ans = score;
    for (int i = 1; i < n - 1; i++) {
      if (s[i] == '0') {
        score++;
      } else {
        score--;
      }
      ans = max(ans, score);
    }
    return ans;
  }
};

int main() {
  assert(Solution().maxScore("011101") == 5);
  assert(Solution().maxScore("00111") == 5);
  assert(Solution().maxScore("1111") == 3);
}