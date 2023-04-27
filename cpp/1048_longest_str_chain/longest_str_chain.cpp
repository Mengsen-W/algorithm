/*
 * @Date: 2023-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-27
 * @FilePath: /algorithm/cpp/1048_longest_str_chain/longest_str_chain.cpp
 */

#include <cassert>
#include <functional>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestStrChain(vector<string> &words) {
    unordered_map<string, int> cnt;
    sort(words.begin(), words.end(), [](const string &a, const string &b) { return a.size() < b.size(); });
    int res = 0;
    for (string word : words) {
      cnt[word] = 1;
      for (int i = 0; i < word.size(); i++) {
        string prev = word.substr(0, i) + word.substr(i + 1);
        if (cnt.count(prev)) {
          cnt[word] = max(cnt[word], cnt[prev] + 1);
        }
      }
      res = max(res, cnt[word]);
    }
    return res;
  }
};

int main() {
  {
    vector<string> words{"a", "b", "ba", "bca", "bda", "bdca"};
    int ans = 4;
    assert(Solution().longestStrChain(words) == ans);
  }

  {
    vector<string> words{"xbc", "pcxbcf", "xb", "cxbc", "pcxbc"};
    int ans = 5;
    assert(Solution().longestStrChain(words) == ans);
  }

  {
    vector<string> words{"abcd", "dbqca"};
    int ans = 1;
    assert(Solution().longestStrChain(words) == ans);
  }
}