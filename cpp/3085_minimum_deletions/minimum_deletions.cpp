#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumDeletions(string word, int k) {
    unordered_map<char, int> cnt;
    for (auto &ch : word) {
      cnt[ch]++;
    }
    int res = word.size();
    for (auto &[_, a] : cnt) {
      int deleted = 0;
      for (auto &[_, b] : cnt) {
        if (a > b) {
          deleted += b;
        } else if (b > a + k) {
          deleted += b - (a + k);
        }
      }
      res = min(res, deleted);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"aabcaba", 0, 3},
      {"dabdcbdcdcd", 2, 2},
      {"aaabaaa", 2, 1},
  };

  for (auto &[word, k, ans] : tests) {
    assert(Solution().minimumDeletions(word, k) == ans);
  }
}