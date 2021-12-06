/*
 * @Date: 2021-12-06 02:16:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-06 02:35:38
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string truncateSentence(string s, int k) {
    int n = s.size();
    int end = 0, count = 0;
    for (int i = 1; i <= n; i++) {
      if (i == n || s[i] == ' ') {
        count++;
        if (count == k) {
          end = i;
          break;
        }
      }
    }
    return s.substr(0, end);
  }
};

int main() {
  assert(Solution().truncateSentence("Hello how are you Contestant", 4) ==
         "Hello how are you");
  assert(Solution().truncateSentence("What is the solution to this problem",
                                     4) == "What is the solution");
  assert(Solution().truncateSentence("chopper is not a tanuki", 5) ==
         "chopper is not a tanuki");
}
