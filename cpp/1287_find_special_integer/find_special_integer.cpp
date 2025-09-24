#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findSpecialInteger(vector<int>& arr) {
    int n = arr.size();
    int span = n / 4 + 1;
    for (int i = 0; i < n; i += span) {
      auto [iter_l, iter_r] = equal_range(arr.begin(), arr.end(), arr[i]);
      if (iter_r - iter_l >= span) {
        return arr[i];
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 2, 6, 6, 6, 6, 7, 10}, 6},
  };
  
  for (auto &[arr, ans] : tests) {
    assert(Solution().findSpecialInteger(arr) == ans);
  }
}