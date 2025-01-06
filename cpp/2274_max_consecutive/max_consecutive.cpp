#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxConsecutive(int bottom, int top, vector<int>& special) {
    special.push_back(bottom - 1);
    special.push_back(top + 1);
    sort(special.begin(), special.end());

    int n = special.size();
    int ans = 0;
    for (int i = 0; i < n - 1; ++i) {
      ans = max(ans, special[i + 1] - special[i] - 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int, vector<int>, int>> tests{
      {2, 9, {4, 6}, 3},
      {6, 8, {7, 6, 8}, 0},
  };

  for (auto& [bottom, top, special, ans] : tests) {
    assert(Solution().maxConsecutive(bottom, top, special) == ans);
  }
}
