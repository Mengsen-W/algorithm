#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumGain(string s, int x, int y) {
    char a = 'a', b = 'b';
    if (x < y) {
      swap(x, y);
      swap(a, b);
    }

    int ans = 0, cnt1 = 0, cnt2 = 0;
    for (char c : s) {
      if (c == a) {
        cnt1++;
      } else if (c == b) {
        if (cnt1) {
          ans += x;
          cnt1--;
        } else {
          cnt2++;
        }
      } else {
        ans += min(cnt1, cnt2) * y;
        cnt1 = 0;
        cnt2 = 0;
      }
    }
    ans += min(cnt1, cnt2) * y;
    return ans;
  }
};

int main() {
  vector<tuple<string, int, int, int>> tests{
      {"cdbcbbaaabab", 4, 5, 19},
      {"aabbaaxybbaabb", 5, 4, 20},
  };

  for (auto &[s, x, y, ans] : tests) {
    assert(Solution().maximumGain(s, x, y) == ans);
  }
}
