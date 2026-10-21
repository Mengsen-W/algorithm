#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int minOperations(string s) {
        int n = static_cast<int>(s.size());
        // 长度 < 2 时必然有序，同时也保证下面的 min/max 取迭代器非空
        if (n < 2) {
            return 0;
        }
        if (is_sorted(s.begin(), s.end())) {
            return 0;
        }
        if (n == 2) {
            return -1;
        }
        char mn = *min_element(s.begin(), s.end());
        char mx = *max_element(s.begin(), s.end());
        if (s[0] == mn || s[n - 1] == mx) {
            return 1;
        }
        if (any_of(s.begin() + 1, s.end() - 1,
                   [&](char c) { return c == mn || c == mx; })) {
            return 2;
        }
        return 3;
    }
};

int main() {
  vector<tuple<string, int>> tests{
      {"dog", 1},  {"card", 2}, {"gf", -1},
      {"", 0},     {"a", 0},    {"abc", 0},
      {"ba", -1},  {"cba", 3},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().minOperations(s) == expected);
  }
}
