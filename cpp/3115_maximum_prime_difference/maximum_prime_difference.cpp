#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

const int MX = 101;
bool not_prime[MX];

auto init = [] {
  not_prime[1] = true;
  for (int i = 2; i * i < MX; i++) {
    if (not_prime[i]) continue;
    for (int j = i * i; j < MX; j += i) {
      not_prime[j] = true;  // j 是质数 i 的倍数
    }
  }
  return 0;
}();

class Solution {
 public:
  int maximumPrimeDifference(vector<int>& nums) {
    int i = 0;
    while (not_prime[nums[i]]) {
      i++;
    }
    int j = nums.size() - 1;
    while (not_prime[nums[j]]) {
      j--;
    }
    return j - i;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{4, 2, 9, 5, 3}, 3},
      {{4, 8, 2, 8}, 0},
  };
  for (auto& [nums, ans] : tests) {
    assert(Solution().maximumPrimeDifference(nums) == ans);
  }
}