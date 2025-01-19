#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countTexts(string pressedKeys) {
    int m = 1000000007;
    vector<long long> dp3 = {1, 1, 2, 4};  // 连续按多次 3 个字母按键对应的方案数
    vector<long long> dp4 = {1, 1, 2, 4};  // 连续按多次 4 个字母按键对应的方案数
    int n = pressedKeys.size();
    for (int i = 4; i < n + 1; ++i) {
      dp3.push_back((dp3[i - 1] + dp3[i - 2] + dp3[i - 3]) % m);
      dp4.push_back((dp4[i - 1] + dp4[i - 2] + dp4[i - 3] + dp4[i - 4]) % m);
    }
    long long res = 1;  // 总方案数
    int cnt = 1;        // 当前字符连续出现的次数
    for (int i = 1; i < n; ++i) {
      if (pressedKeys[i] == pressedKeys[i - 1]) {
        ++cnt;
      } else {
        // 对按键对应字符数量讨论并更新总方案数
        if (pressedKeys[i - 1] == '7' || pressedKeys[i - 1] == '9') {
          res *= dp4[cnt];
        } else {
          res *= dp3[cnt];
        }
        res %= m;
        cnt = 1;
      }
    }
    // 更新最后一段连续字符子串对应的方案数
    if (pressedKeys[n - 1] == '7' || pressedKeys[n - 1] == '9') {
      res *= dp4[cnt];
    } else {
      res *= dp3[cnt];
    }
    res %= m;
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"22233", 8},
      {"222222222222222222222222222222222222", 82876089},
  };

  for (auto &[pressedKeys, ans] : tests) {
    assert(Solution().countTexts(pressedKeys) == ans);
  }
}