#include <cassert>
#include <tuple>
#include <vector>

class Solution {
  public:
    char kthCharacter(int k) {
      int ans = 0;
      int t;
      while (k != 1) {
        t = std::__lg(k);
        if ((1 << t) == k) {
          t--;
        }
        k = k - (1 << t);
        ans++;
      }
      return 'a' + ans;
    }
};

int main() {
  std::vector<std::tuple<int, char>> tests {
    {5, 'b'},
    {10, 'c'},
  };

  for (auto &[k ,ans] : tests) {
    assert(Solution().kthCharacter(k) == ans);
  }
}
