#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> smallestSubarrays(vector<int>& nums) {
        int n = nums.size();
        vector<int> pos(31, -1);
        vector<int> ans(n);
        for (int i = n - 1; i >= 0; --i) {
            int j = i;
            for (int bit = 0; bit < 31; ++bit) {
                if (!(nums[i] & (1 << bit))) {
                    if (pos[bit] != -1) {
                        j = max(j, pos[bit]);
                    }
                }
                else {
                    pos[bit] = i;
                }
            }
            ans[i] = j - i + 1;
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 0, 2, 1, 3}, {3, 3, 2, 2, 1}},
      {{1, 2}, {2, 1}},
  };

  for (auto& [nums, expect] : tests) {
    assert(Solution().smallestSubarrays(nums) == expect);
  }
}