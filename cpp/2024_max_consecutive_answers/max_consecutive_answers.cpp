/*
 * @Date: 2022-03-29 01:59:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-29 02:16:08
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxConsecutiveChar(string& answerKey, int k, char ch) {
    int n = answerKey.length();
    int ans = 0;
    for (int left = 0, right = 0, sum = 0; right < n; right++) {
      sum += answerKey[right] != ch;
      while (sum > k) {
        sum -= answerKey[left++] != ch;
      }
      ans = max(ans, right - left + 1);
    }
    return ans;
  }

  int maxConsecutiveAnswers(string answerKey, int k) {
    return max(maxConsecutiveChar(answerKey, k, 'T'), maxConsecutiveChar(answerKey, k, 'F'));
  }
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"TTFF", 2, 4},
      {"TFFT", 1, 3},
      {"TTFTTFTT", 1, 5},
  };

  for (auto& [answerKey, k, ans] : tests) {
    assert(Solution().maxConsecutiveAnswers(answerKey, k) == ans);
  }
}