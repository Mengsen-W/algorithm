/*
 * @Date: 2021-11-08 00:08:29
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-10
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string getHint(string secret, string guess) {
    int bulls = 0;
    vector<int> cntS(10), cntG(10);
    int secret_len = secret.size();
    for (int i = 0; i < secret_len; ++i)
      if (secret[i] == guess[i]) {
        ++bulls;
      } else {
        ++cntS[secret[i] - '0'];
        ++cntG[guess[i] - '0'];
      }

    int cows = 0;
    for (int i = 0; i < 10; ++i) cows += min(cntS[i], cntG[i]);

    return to_string(bulls) + "A" + to_string(cows) + "B";
  }
};

int main() {
  vector<tuple<string, string, string>> tests{
      {"1807", "7810", "1A3B"},
      {"1123", "0111", "1A1B"},
      {"1", "0", "0A0B"},
      {"1", "1", "1A0B"},
  };

  for (auto &[secret, guess, ans] : tests) {
    assert(Solution().getHint(secret, guess) == ans);
  }
}