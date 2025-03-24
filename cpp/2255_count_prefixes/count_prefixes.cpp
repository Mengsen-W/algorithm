#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countPrefixes(vector<string>& words, string s) {
    int res = 0;  // 符合要求字符串个数
    // 判断 word 是否是 s 的前缀
    auto isPrefix = [&](const string& word) -> bool {
      if (s.size() < word.size()) {
        return false;
      }
      for (int i = 0; i < word.size(); ++i) {
        if (word[i] != s[i]) {
          return false;
        }
      }
      return true;
    };

    for (const string& word : words) {
      if (isPrefix(word)) {
        ++res;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, string, int>> tests{
      {{"a", "b", "c", "ab", "bc", "abc"}, "abc", 3},
      {{"a", "a"}, "aa", 2},
  };

  for (auto &[words, s, ans] : tests) {
    assert(Solution().countPrefixes(words, s) == ans);
  }
}