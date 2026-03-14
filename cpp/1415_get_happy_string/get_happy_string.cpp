#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  string getHappyString(int n, int k) {
    vector<char> chs = {'a', 'b', 'c'};
    string res;
    if (k > 3 * (1 << (n - 1))) {
      return res;
    }
    for (int i = 0; i < n; i++) {
      int count = 1 << (n - i - 1);
      for (char c : chs) {
        if (!res.empty() && res.back() == c) {
          continue;
        }
        if (k <= count) {
          res.push_back(c);
          break;
        }
        k -= count;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, string>> tests{
      {1, 3, "c"},
      {1, 4, ""},
      {3, 9, "cab"},
      {2, 7, ""},
      {10, 100, "abacbabacb"},
  };

  for (auto [n, k, expected] : tests) {
    assert(Solution().getHappyString(n, k) == expected);
  }
}