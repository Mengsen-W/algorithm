#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumBeauty(vector<int>& flowers, long long newFlowers, int target, int full, int partial) {
    int n = flowers.size();
    for (int& x : flowers) {
      x = min(x, target);
    }
    sort(flowers.begin(), flowers.end(), greater<int>());
    long long sum = accumulate(flowers.begin(), flowers.end(), 0LL);
    long long ans = 0;
    if (static_cast<long long>(target) * n - sum <= newFlowers) {
      ans = static_cast<long long>(full) * n;
    }

    long long pre = 0;
    int ptr = 0;
    for (int i = 0; i < n; ++i) {
      if (i != 0) {
        pre += flowers[i - 1];
      }
      if (flowers[i] == target) {
        continue;
      }
      long long rest = newFlowers - (static_cast<long long>(target) * i - pre);
      if (rest < 0) {
        break;
      }
      while (!(ptr >= i && static_cast<long long>(flowers[ptr]) * (n - ptr) - sum <= rest)) {
        sum -= flowers[ptr];
        ++ptr;
      }
      rest -= static_cast<long long>(flowers[ptr]) * (n - ptr) - sum;
      ans = max(ans, static_cast<long long>(full) * i +
                         static_cast<long long>(partial) *
                             (min(flowers[ptr] + rest / (n - ptr), static_cast<long long>(target) - 1)));
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int, int, int>> tests{
      {{1, 3, 1, 1}, 7, 6, 12, 1, 14},
      {{2, 4, 5, 3}, 10, 5, 2, 6, 30},
  };

  for (auto& [flowers, newFlowers, target, full, partial, ans] : tests) {
    assert(Solution().maximumBeauty(flowers, newFlowers, target, full, partial) == ans);
  }
}