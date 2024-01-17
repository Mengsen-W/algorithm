/*
 * @Date: 2024-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-17
 * @FilePath: /algorithm/cpp/2744_maximum_number_of_string_pairs/maximum_number_of_string_pairs.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumNumberOfStringPairs(vector<string>& words) {
    int n = words.size();
    int ans = 0;
    unordered_set<int> seen;
    for (int i = 0; i < n; ++i) {
      if (seen.count(words[i][1] * 100 + words[i][0])) {
        ++ans;
      }
      seen.insert(words[i][0] * 100 + words[i][1]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"cd", "ac", "dc", "ca", "zz"}, 2},
      {{"ab", "ba", "cc"}, 1},
      {{"aa", "ab"}, 0},
  };

  for (auto& [words, ans] : tests) {
    assert(Solution().maximumNumberOfStringPairs(words) == ans);
  }
}