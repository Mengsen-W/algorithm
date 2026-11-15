#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numOfSubarrays(vector<int>& arr) {
    const int MODULO = 1000000007;
    int odd = 0, even = 1;
    int subarrays = 0;
    int sum = 0;
    int length = arr.size();
    for (int i = 0; i < length; i++) {
      sum += arr[i];
      subarrays = (subarrays + (sum % 2 == 0 ? odd : even)) % MODULO;
      if (sum % 2 == 0) {
        even++;
      } else {
        odd++;
      }
    }
    return subarrays;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 5}, 4}, {{2, 4, 6}, 0}, {{1, 2, 3, 4, 5, 6, 7}, 16}, {{100, 100, 99, 99}, 4}, {{7}, 1},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().numOfSubarrays(nums) == expected);
  }
}
