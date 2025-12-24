#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumBoxes(vector<int>& apple, vector<int>& capacity) {
    int sum = accumulate(apple.begin(), apple.end(), 0);
    sort(capacity.begin(), capacity.end(), greater<int>());

    int need = 0;
    while (sum > 0) {
      sum -= capacity[need];
      need += 1;
    }

    return need;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 3, 2}, {4, 3, 1, 5, 2}, 2},
      {{5, 5, 5}, {2, 4, 2, 7}, 4},
  };

  for (auto& [apple, capacity, expect] : tests) {
    assert(Solution().minimumBoxes(apple, capacity) == expect);
  }
}