#include <cassert>
#include <string>
#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
 public:
  int numberOfSpecialChars(string word) {
    unordered_set<char> s(word.begin(), word.end());
    int ans = 0;
    for (char c = 'a'; c <= 'z'; c++) {
      if (s.count(c) && s.count(c - 'a' + 'A')) {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"aaAbcBC", 3},
      {"abc", 0},
      {"abBCab", 1},
  };

  for (auto [word, expected] : tests) {
    assert(Solution().numberOfSpecialChars(word) == expected);
  }
}