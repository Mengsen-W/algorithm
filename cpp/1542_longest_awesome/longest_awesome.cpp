/*
 * @Date: 2024-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-20
 * @FilePath: /algorithm/cpp/1542_longest_awesome/longest_awesome.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestAwesome(string s) {
    int n = s.size();
    unordered_map<int, int> prefix = {{0, -1}};
    int ans = 0;
    int sequence = 0;
    for (int j = 0; j < n; ++j) {
      int digit = s[j] - '0';
      sequence ^= (1 << digit);
      if (prefix.count(sequence)) {
        ans = max(ans, j - prefix[sequence]);
      } else {
        prefix[sequence] = j;
      }
      for (int k = 0; k < 10; ++k) {
        if (prefix.count(sequence ^ (1 << k))) {
          ans = max(ans, j - prefix[sequence ^ (1 << k)]);
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"3242415", 5},
      {"12345678", 1},
      {"213123", 6},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().longestAwesome(s) == ans);
  }
}