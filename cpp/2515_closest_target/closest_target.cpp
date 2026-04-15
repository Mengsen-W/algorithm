#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int closestTarget(vector<string>& words, string target, int startIndex) {
    int ans = words.size();
    int n = words.size();

    for (int i = 0; i < n; ++i) {
      if (words[i] == target) {
        int dist = abs(i - startIndex);
        ans = min(ans, min(dist, n - dist));
      }
    }

    return ans < n ? ans : -1;
  }
};

int main() {
  vector<tuple<vector<string>, string, int, int>> tests{
      {{"hello", "i", "am", "leetcode", "hello"}, "hello", 1, 1},
      {{"a", "b", "leetcode"}, "leetcode", 0, 1},
      {{"i", "eat", "leetcode"}, "ate", 0, -1},
  };

  for (auto& [words, target, startIndex, ans] : tests) {
    assert(Solution().closestTarget(words, target, startIndex) == ans);
  }
}
