#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int maximumLength(vector<int>& nums) {
        int res = 0;
        vector<vector<int>> patterns = {{0, 0}, {0, 1}, {1, 0}, {1, 1}};
        for (auto& pattern : patterns) {
            int cnt = 0;
            for (int num : nums) {
                if (num % 2 == pattern[cnt % 2]) {
                    cnt++;
                }
            }
            res = max(res, cnt);
        }
        return res;
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4}, 4},
      {{1, 2, 1, 1, 2, 1, 2}, 6},
      {{1, 3}, 2},
  };
  
  for (auto &[nums, expected] : tests) {
    assert(Solution().maximumLength(nums) == expected);
  }
}