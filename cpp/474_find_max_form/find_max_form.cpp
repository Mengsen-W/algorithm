#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int findMaxForm(vector<string>& strs, int m, int n) {
    vector<vector<int>> dp(m + 1, vector<int>(n + 1, 0));
    for (string str : strs) {
      int zeroNum = 0, oneNum = 0;
      for (char c : str) {  // 统计每个字符串的0和1的数量
        if (c == '0') {
          zeroNum++;
        } else {
          oneNum++;
        }
      }
      for (int i = m; i >= zeroNum; i--) {  // 01背包罢了, 但是重量有两个维度
        for (int j = n; j >= oneNum; j--) {
          dp[i][j] = max(dp[i][j], dp[i - zeroNum][j - oneNum] + 1);
        }  // 不装 or 装得下, 取最大值
      }
    }
    return dp[m][n];
  }
};

int main() {
  vector<tuple<vector<string>, int, int, int>> tests{
      {{"10", "0001", "111001", "1", "0"}, 5, 3, 4},
      {{"01", "0", "1"}, 1, 1, 2},
  };

  for (auto [strs, m, n, ans] : tests) {
    assert(Solution().findMaxForm(strs, m, n) == ans);
  }
}