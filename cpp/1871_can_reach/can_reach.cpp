#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool canReach(string s, int minJump, int maxJump) {
    int n = s.size();
    vector<int> f(n), pre(n);
    f[0] = 1;
    // 由于我们从 i=minJump 开始动态规划，因此需要将 [0,minJump) 这部分的前缀和预处理出来
    for (int i = 0; i < minJump; ++i) {
      pre[i] = 1;
    }
    for (int i = minJump; i < n; ++i) {
      int left = i - maxJump, right = i - minJump;
      if (s[i] == '0') {
        int total = pre[right] - (left <= 0 ? 0 : pre[left - 1]);
        f[i] = (total != 0);
      }
      pre[i] = pre[i - 1] + f[i];
    }
    return f[n - 1];
  }
};

int main() {
  vector<tuple<string, int, int, bool>> tests{
      {"011010", 2, 3, true},
      {"01101110", 2, 3, false},
  };

  for (auto& [s, minJump, maxJump, expect] : tests) {
    assert(Solution().canReach(s, minJump, maxJump) == expect);
  }
}