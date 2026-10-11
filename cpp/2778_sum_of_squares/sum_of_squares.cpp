#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int sumOfSquares(vector<int>& nums) {
        int n = nums.size();
        int ans = 0;
        for (int i = 1; i * i <= n; ++i) {
            if (n % i == 0) {
                ans += nums[i - 1] * nums[i - 1];
                if (i * i != n) {
                    ans += nums[n / i - 1] * nums[n / i - 1];
                }
            }
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4}, 21},
      {{2, 7, 1, 19, 18, 3}, 63},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().sumOfSquares(nums) == expected);
  }
}
