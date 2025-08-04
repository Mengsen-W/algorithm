#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int totalFruit(vector<int>& fruits) {
    int n = fruits.size();
    unordered_map<int, int> cnt;

    int left = 0, ans = 0;
    for (int right = 0; right < n; ++right) {
      ++cnt[fruits[right]];
      while (cnt.size() > 2) {
        auto it = cnt.find(fruits[left]);
        --it->second;
        if (it->second == 0) {
          cnt.erase(it);
        }
        ++left;
      }
      ans = max(ans, right - left + 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1}, 3},
      {{0, 1, 2, 2}, 3},
      {{1, 2, 3, 2, 2}, 4},
      {{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4}, 5},
  };

  for (auto& [fruits, expect] : tests) {
    assert(Solution().totalFruit(fruits) == expect);
  }
}