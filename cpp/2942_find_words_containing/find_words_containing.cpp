#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findWordsContaining(vector<string>& words, char x) {
    vector<int> res;
    int n = words.size();
    for (int i = 0; i < n; ++i) {
      if (words[i].find(x) != string::npos) {
        res.push_back(i);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, char, vector<int>>> tests{
      {{"leet", "code"}, 'e', {0, 1}},
      {{"abc", "bcd", "aaaa", "cbc"}, 'a', {0, 2}},
      {{"abc", "bcd", "aaaa", "cbc"}, 'z', {}},
  };

  for (auto& [words, x, ans] : tests) {
    assert(Solution().findWordsContaining(words, x) == ans);
  }
}