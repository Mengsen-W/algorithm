#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  string mapWordWeights(vector<string>& words, vector<int>& weights) {
    string ans;
    ans.reserve(words.size());
    for (string word : words) {
      int s = 0;
      for (char c : word) {
        s += weights[c - 'a'];
      }
      ans += 'z' - s % 26;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, vector<int>, string>> tests{
      {
          {"abcd", "def", "xyz"},
          {5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2},
          "rij",
      },
      {
          {"a", "b", "c"},
          {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
          "yyy",
      },
  };

  for (auto& [words, weights, expected] : tests) {
    assert(Solution().mapWordWeights(words, weights) == expected);
  }
  return 0;
}