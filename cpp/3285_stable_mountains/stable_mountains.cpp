#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> stableMountains(vector<int>& height, int threshold) {
    vector<int> result;
    for (int i = 1; i < height.size(); i++) {
      if (height[i - 1] > threshold) {
        result.push_back(i);
      }
    }
    return result;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{1, 2, 3, 4, 5}, 2, {3, 4}},
      {{10, 1, 10, 1, 10}, 3, {1, 3}},
      {{10, 1, 10, 1, 10}, 10, {}},
  };
  
  for (auto &[height, threshold, ans] : tests) {
    assert(Solution().stableMountains(height, threshold) == ans);
  }
}
