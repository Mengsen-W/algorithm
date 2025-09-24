#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> getLongestSubsequence(vector<string>& words, vector<int>& groups) {
    vector<string> ans;
    int n = words.size();
    for (int i = 0; i < n; i++) {
      if (i == 0 || groups[i] != groups[i - 1]) {
        ans.emplace_back(words[i]);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, vector<int>, vector<string>>> tests{
      {{"e", "a", "b"}, {0, 0, 1}, {"e", "b"}},
      {{"a", "b", "c", "d"}, {1, 0, 1, 1}, {"a", "b", "c"}},
  };

  for (auto& [words, groups, ans] : tests) {
    assert(Solution().getLongestSubsequence(words, groups) == ans);
  }
}