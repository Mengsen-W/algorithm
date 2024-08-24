#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int findPermutationDifference(string s, string t) {
    unordered_map<char, int> char2index;
    for (int i = 0; i < s.length(); ++i) {
      char2index[s[i]] = i;
    }
    int sum = 0;
    for (int i = 0; i < t.length(); ++i) {
      sum += abs(i - char2index[t[i]]);
    }
    return sum;
  }
};

int main() {
  vector<tuple<string, string, int>> tests{
      {"abc", "bac", 2},
      {"abcde", "edbac", 12},
  };

  for (auto &[s, t, ans] : tests) {
    assert(Solution().findPermutationDifference(s, t) == ans);
  }
}
