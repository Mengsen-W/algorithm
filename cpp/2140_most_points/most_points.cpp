#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long mostPoints(vector<vector<int>>& questions) {
    int n = questions.size();
    vector<long long> dp(n + 1);  // 解决每道题及以后题目的最高分数
    for (int i = n - 1; i >= 0; --i) {
      dp[i] = max(dp[i + 1], questions[i][0] + dp[min(n, i + questions[i][1] + 1)]);
    }
    return dp[0];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, long long>> tests{
      {{{3, 2}, {4, 3}, {4, 4}, {2, 5}}, 5},
      {{{1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}}, 7},
  };

  for (auto& [questions, ans] : tests) {
    assert(Solution().mostPoints(questions) == ans);
  }
}