#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int similarPairs(vector<string>& words) {
    int res = 0;
    unordered_map<int, int> cnt;
    for (const string& word : words) {
      int state = 0;
      for (char c : word) {
        state |= 1 << (c - 'a');
      }
      res += cnt[state];
      cnt[state]++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"aba", "aabb", "abcd", "bac", "aabc"}, 2},
      {{"aabb", "ab", "ba"}, 3},
      {{"nba", "cba", "dba"}, 0},
  };
  
  for (auto &[words, ans] : tests) {
    assert(Solution().similarPairs(words) == ans);
  }
}