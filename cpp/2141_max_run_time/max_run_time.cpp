#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxRunTime(int n, vector<int>& batteries) {
    long long left = 0, right = accumulate(batteries.begin(), batteries.end(), 0LL) / n, ans = 0;
    while (left <= right) {
      long long mid = (left + right) / 2;
      long long total = 0;
      for (int cap : batteries) {
        total += min(static_cast<long long>(cap), mid);
      }
      if (total >= n * mid) {
        ans = mid;
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<int>, long long>> tests{
      {2, {3, 3, 3}, 4},
      {2, {1, 1, 1, 1}, 2},
  };

  for (auto &[n, batteries, ans] : tests) {
    assert(Solution().maxRunTime(n, batteries) == ans);
  }
}