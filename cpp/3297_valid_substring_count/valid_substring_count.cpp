#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long validSubstringCount(string word1, string word2) {
    vector<int> diff(26, 0);
    for (auto c : word2) {
      diff[c - 'a']--;
    }

    long long res = 0;
    int cnt = count_if(diff.begin(), diff.end(), [](int c) { return c < 0; });
    auto update = [&](int c, int add) {
      diff[c] += add;
      if (add == 1 && diff[c] == 0) {
        // 表明 diff[c] 由 -1 变为 0
        cnt--;
      } else if (add == -1 && diff[c] == -1) {
        // 表明 diff[c] 由 0 变为 -1
        cnt++;
      }
    };

    for (int l = 0, r = 0; l < word1.size(); l++) {
      while (r < word1.size() && cnt > 0) {
        update(word1[r] - 'a', 1);
        r++;
      }
      if (cnt == 0) {
        res += word1.size() - r + 1;
      }
      update(word1[l] - 'a', -1);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string, long long>> tests{
      {"bcca", "abc", 1},
      {"abcabc", "abc", 10},
      {"abcabc", "aaabc", 0},
  };
  
  for (auto &[word1, word2, ans] : tests) {
    assert(Solution().validSubstringCount(word1, word2) == ans);
  }
}