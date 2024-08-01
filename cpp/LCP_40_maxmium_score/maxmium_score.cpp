
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxmiumScore(vector<int>& cards, int cnt) {
    int val[1005];
    for (int i = 0; i < cards.size(); i++) {
      val[cards[i]]++;
    }

    int ans = 0;
    int tmp = 0;
    int ed = -1;
    int odd, even = -1;
    for (int i = 1000; i >= 1; i--) {
      if (val[i] == 0) {
        continue;
      }

      if (val[i] > cnt) {
        tmp += cnt * i;
        cnt = 0;
      } else {
        tmp += val[i] * i;
        cnt -= val[i];
        val[i] = 0;
      }

      if (i & 1) {
        odd = i;
      } else {
        even = i;
      }

      if (!cnt) {
        if (val[i] > 0) {
          ed = i;
        } else {
          ed = i - 1;
        }
        break;
      }
    }

    if (!(tmp & 1)) {
      return tmp;
    }

    for (int i = ed; i >= 1; i--) {
      if (val[i] == 0) {
        continue;
      }

      if (i & 1) {
        if (even != -1) {
          ans = max(ans, tmp - even + i);
        }
      } else {
        if (odd != -1) {
          ans = max(ans, tmp - odd + i);
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 8, 9}, 3, 18},
      {{3, 3, 1}, 1, 0},
  };

  for (auto &[cards, cnt, ans] : tests) {
    assert(Solution().maxmiumScore(cards,  cnt) == ans);
  }
}