#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    bool doesValidArrayExist(vector<int>& derived) {
        return reduce(derived.begin(), derived.end(), 0, bit_xor()) == 0;
    }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 1, 0}, true},
      {{1, 1}, true},
      {{1, 0}, false},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().doesValidArrayExist(nums) == expected);
  }
}