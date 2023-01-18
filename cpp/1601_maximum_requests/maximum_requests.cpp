/*
 * @Date: 2022-02-28 02:47:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-28 03:01:31
 * @FilePath: /algorithm/1601_maximum_requests/maximum_requests.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution1 {
 private:
  vector<int> delta;
  int ans = 0, cnt = 0, zero, n;

 public:
  void dfs(vector<vector<int>> &requests, int pos) {
    if (pos == requests.size()) {
      if (zero == n) {
        ans = max(ans, cnt);
      }
      return;
    }

    // 不选 requests[pos]
    dfs(requests, pos + 1);

    // 选 requests[pos]
    int z = zero;
    ++cnt;
    auto &r = requests[pos];
    int x = r[0], y = r[1];
    zero -= delta[x] == 0;
    --delta[x];
    zero += delta[x] == 0;
    zero -= delta[y] == 0;
    ++delta[y];
    zero += delta[y] == 0;
    dfs(requests, pos + 1);
    --delta[y];
    ++delta[x];
    --cnt;
    zero = z;
  }

  int maximumRequests(int n, vector<vector<int>> &requests) {
    delta.resize(n);
    zero = n;
    this->n = n;
    dfs(requests, 0);
    return ans;
  }
};

class Solution2 {
 public:
  int maximumRequests(int n, vector<vector<int>> &requests) {
    vector<int> delta(n);
    int ans = 0, m = requests.size();
    for (int mask = 0; mask < (1 << m); ++mask) {
      int cnt = __builtin_popcount(mask);
      if (cnt <= ans) {
        continue;
      }
      fill(delta.begin(), delta.end(), 0);
      for (int i = 0; i < m; ++i) {
        if (mask & (1 << i)) {
          ++delta[requests[i][0]];
          --delta[requests[i][1]];
        }
      }
      if (all_of(delta.begin(), delta.end(), [](int x) { return x == 0; })) {
        ans = cnt;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> input{{0, 1}, {1, 0}, {0, 1}, {1, 2}, {2, 0}, {3, 4}};
    assert(Solution1().maximumRequests(5, input) == 5);
    assert(Solution2().maximumRequests(5, input) == 5);
  }

  {
    vector<vector<int>> input{{0, 0}, {1, 2}, {2, 1}};
    assert(Solution1().maximumRequests(5, input) == 3);
    assert(Solution2().maximumRequests(5, input) == 3);
  }

  {
    vector<vector<int>> input{{0, 3}, {3, 1}, {1, 2}, {2, 0}};
    assert(Solution1().maximumRequests(5, input) == 4);
    assert(Solution2().maximumRequests(5, input) == 4);
  }
}