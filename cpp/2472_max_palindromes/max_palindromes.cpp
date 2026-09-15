#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int maxPalindromes(string s, int k) {
        int n = s.size();
        int ans = 0, start = 0;

        auto check = [&](int l, int r) {
            while (l < r) {
                if (s[l++] != s[r--]) {
                    return false;
                }
            }
            return true;
        };

        for (int r = k - 1; r < n; ++r) {
            int l = r - k + 1;
            if (l >= start && check(l, r)) {
                ++ans;
                start = r + 1;
                continue;
            }

            l = r - k;
            if (l >= start && check(l, r)) {
                ++ans;
                start = r + 1;
            }
        }

        return ans;
    }
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"abaccdbbd", 3, 2},
      {"adbcda", 2, 0},
  };

  for (auto &[s, k, ans] : tests) {
    assert(Solution().maxPalindromes(s, k) == ans);
  }
  return 0;
}
