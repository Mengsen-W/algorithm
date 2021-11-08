/*
 * @Date: 2021-11-08 00:08:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-08 00:12:43
 */

#include <cassert>
#include <string>
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
  assert(Solution().getHint("1807", "7810") == "1A3B");
  assert(Solution().getHint("1123", "0111") == "1A1B");
  assert(Solution().getHint("1", "0") == "0A0B");
  assert(Solution().getHint("1", "1") == "1A0B");
}