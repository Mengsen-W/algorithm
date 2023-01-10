/*
 * @Date: 2023-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-10
 * @FilePath: /algorithm/753_crack_safe/crack_safe.cpp
 */

#include <cassert>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
 private:
  unordered_set<int> seen;
  string ans;
  int highest;
  int k;

 public:
  void dfs(int node) {
    for (int x = 0; x < k; ++x) {
      int nei = node * 10 + x;
      if (!seen.count(nei)) {
        seen.insert(nei);
        dfs(nei % highest);
        ans += (x + '0');
      }
    }
  }

  string crackSafe(int n, int k) {
    highest = pow(10, n - 1);
    this->k = k;
    dfs(0);
    ans += string(n - 1, '0');
    return ans;
  }
};

int main() {
  {
    int n = 1;
    int k = 2;
    string ans{"10"};
    assert(Solution().crackSafe(n, k) == ans);
  }

  {
    int n = 2;
    int k = 2;
    string ans{"01100"};
    assert(Solution().crackSafe(n, k) == ans);
  }
}
