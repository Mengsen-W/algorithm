#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int minOperations(vector<int>& nums) {
        const int n = nums.size();
        if (n == 1) {
            return 0;
        }

        const int start = find(nums.begin(), nums.end(), 0) - nums.begin();
        bool asc = true;
        bool desc = true;

        for (int i = 1; i < n; i++) {
            const int value = nums[(start + i) % n];

            if (value != i) {
                asc = false;
            }
            if (value != n - i) {
                desc = false;
            }

            if (!asc && !desc) {
                return -1;
            }
        }

        if (asc) {
            return min(start, n - start + 2);
        }

        return min(n - start, start + 2);
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 2, 1}, 2},
      {{1, 0, 2}, 2},
      {{2, 0, 1, 3}, -1},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().minOperations(nums) == expected);
  }
}