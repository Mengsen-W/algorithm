#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> resultArray(vector<int>& nums) {
    int n = nums.size();
    vector<int> arr1, arr2;
    arr1.push_back(nums[0]);
    arr2.push_back(nums[1]);
    for (int i = 2; i < n; i++) {
      if (arr1.back() > arr2.back()) {
        arr1.push_back(nums[i]);
      } else {
        arr2.push_back(nums[i]);
      }
    }
    arr1.insert(arr1.end(), arr2.begin(), arr2.end());
    return arr1;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{2, 1, 3}, {2, 3, 1}},
      {{5, 4, 3, 8}, {5, 3, 4, 8}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().resultArray(nums) == ans);
  }
}