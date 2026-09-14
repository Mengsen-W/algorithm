#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool isRectangleOverlap(vector<int>& rec1, vector<int>& rec2) {
    return (min(rec1[2], rec2[2]) > max(rec1[0], rec2[0]) && min(rec1[3], rec2[3]) > max(rec1[1], rec2[1]));
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, bool>> tests{
      {{0, 0, 2, 2}, {1, 1, 3, 3}, true},
      {{0, 0, 1, 1}, {1, 0, 2, 1}, false},
      {{0, 0, 1, 1}, {2, 2, 3, 3}, false},
  };

  for (auto& [rec1, rec2, ans] : tests) {
    assert(Solution().isRectangleOverlap(rec1, rec2) == ans);
  }
  return 0;
}