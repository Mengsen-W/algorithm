#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool stoneGameIX(vector<int> stones) {
    int cnt0 = 0, cnt1 = 0, cnt2 = 0;
    for (int val : stones) {
      if (int type = val % 3; type == 0) {
        ++cnt0;
      } else if (type == 1) {
        ++cnt1;
      } else {
        ++cnt2;
      }
    }
    if (cnt0 % 2 == 0) {
      return cnt1 >= 1 && cnt2 >= 1;
    }
    return cnt1 - cnt2 > 2 || cnt2 - cnt1 > 2;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{2, 1}, true},
      {{2}, false},
      {{5, 1, 2, 4, 3}, false},
  };

  for (auto& [stones, ans] : tests) {
    assert(Solution().stoneGameIX(stones) == ans);
  }
}
