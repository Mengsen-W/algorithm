#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool check(int x, vector<int>& position, int m) {
    int pre = position[0], cnt = 1;
    for (int i = 1; i < position.size(); ++i) {
      if (position[i] - pre >= x) {
        pre = position[i];
        cnt += 1;
      }
    }
    return cnt >= m;
  }

  int maxDistance(vector<int>& position, int m) {
    sort(position.begin(), position.end());
    int left = 1, right = position.back() - position[0], ans = -1;
    while (left <= right) {
      int mid = (left + right) / 2;
      if (check(mid, position, m)) {
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
  vector<tuple<vector<int>, int, int>> tests {
    {{1, 2, 3, 4, 7}, 3, 3},
		{{5, 4, 3, 2, 1, 1000000000}, 2, 999999999},
  };
  
  for (auto &[position, m, ans] : tests) {
    assert(Solution().maxDistance(position, m) == ans);
  }
}
