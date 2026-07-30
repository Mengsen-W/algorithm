#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minimumPushes(string word) {
    int n = word.size();
    int m = (n - 1) / 8 + 1;  // 最大需要按多少次 计算向上取整
    // 8 * 1 + 8 * 2 + ... + 8 * (m - 1) = 8 * (m - 1 + 1) * (m - 1) / 2 = 4 * m * (m - 1)
    // 最后一层字母数量 ( n - (m - 1) * 8 ) 按键次数 m ，总按键次数 (n - (m - 1) * 8) * m
    return m * (m - 1) * 4 + (n - (m - 1) * 8) * m;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abcde", 5},
      {"xycdefghij", 12},
  };

  for (auto &[word, ans] : tests) {
    assert(Solution().minimumPushes(word) == ans);
  }
}
