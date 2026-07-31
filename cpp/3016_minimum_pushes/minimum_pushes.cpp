#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minimumPushes(string word) {
    // 贪心可重复，所以先放最大的
    std::vector<int> freq(26);
    for (char c : word) {
      freq[c - 'a']++;
    }
    std::sort(freq.begin(), freq.end(), std::greater<>());
    int ans = 0;
    // 这里按照字母频率从大到小排列，每 8 个一组，每组字母贪心的乘频率
    for (int i = 0; i < 26 && freq[i] > 0; i++) {
      ans += freq[i] * (i / 8 + 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abcde", 5},
      {"xyzxyzxyzxyz", 12},
      {"aabbccddeeffgghhiiiiii", 24},
  };

  for (auto &[word, ans] : tests) {
    assert(Solution().minimumPushes(word) == ans);
  }
}
