#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int getSteps(int curr, long n) {
    int steps = 0;
    long first = curr;
    long last = curr;
    while (first <= n) {
      steps += min(last, n) - first + 1;
      first = first * 10;
      last = last * 10 + 9;
    }
    return steps;
  }

  int findKthNumber(int n, int k) {
    int curr = 1;
    k--;
    while (k > 0) {
      int steps = getSteps(curr, n);
      if (steps <= k) {
        k -= steps;
        curr++;
      } else {
        curr = curr * 10;
        k--;
      }
    }
    return curr;
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {13, 2, 10},
      {1, 1, 1},
  };

  for (auto [n, k, expected] : tests) {
    assert(Solution().findKthNumber(n, k) == expected);
  }
}