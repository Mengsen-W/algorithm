#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> targetIndices(vector<int>& nums, int target) {
        int cnt1 = 0;   // 小于 target 的元素数量
        int cnt2 = 0;   // 等于 target 的元素数量
        for (const int num: nums){
            if (num < target){
                ++cnt1;
            }
            else if (num == target){
                ++cnt2;
            }
        }
        vector<int> res;   // 下标数组
        for (int i = cnt1; i < cnt1 + cnt2; ++i){
            res.push_back(i);
        }
        return res;
    }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{1, 2, 5, 2, 3}, 2, {1, 2}},
      {{1, 2, 5, 2, 3}, 3, {3}},
      {{1, 2, 5, 2, 3}, 5, {4}},
      {{1, 2, 5, 2, 3}, 4, {}},
  };

  for (auto [nums, target, expected] : tests) {
    assert(Solution().targetIndices(nums, target) == expected);
  }
}