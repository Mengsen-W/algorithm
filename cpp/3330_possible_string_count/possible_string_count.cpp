#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int possibleStringCount(string word) {
    int n = word.size(), ans = 1;
    for (int i = 1; i < n; ++i) {
      if (word[i - 1] == word[i]) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abbcccc", 5},
      {"abcd", 1},
      {"aaaa", 4},
  };

  for (auto &[word, expected] : tests) {
    assert(expected == Solution().possibleStringCount(word));
  }
}