#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int diagonalPrime(vector<vector<int>>& nums) {
    int n = nums.size(), res = 0;
    for (int i = 0; i < n; i++) {
      if (isPrime(nums[i][i])) {
        res = max(res, nums[i][i]);
      }
      if (isPrime(nums[i][n - i - 1])) {
        res = max(res, nums[i][n - i - 1]);
      }
    }
    return res;
  }

  bool isPrime(int num) {
    if (num == 1) {
      return false;
    }
    int factor = 2;
    while (factor * factor <= num) {
      if (num % factor == 0) {
        return false;
      }
      factor++;
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 3}, {5, 6, 7}, {9, 10, 11}}, 11},
      {{{1, 2, 3}, {5, 17, 7}, {9, 11, 10}}, 17},
  };
  
  for (auto &[nums, ans] : tests) {
    assert(Solution().diagonalPrime(nums) == ans);
  }
}