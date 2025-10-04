#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxArea(vector<int>& height) {
    int ans = 0, left = 0, right = height.size() - 1;
    while (left < right) {
      int area = (right - left) * min(height[left], height[right]);
      ans = max(ans, area);
      height[left] < height[right] ? left++ : right--;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 8, 6, 2, 5, 4, 8, 3, 7}, 49},
      {{1, 1}, 1},
  };

  for (auto& [height, ans] : tests) {
    assert(Solution().maxArea(height) == ans);
  }
}