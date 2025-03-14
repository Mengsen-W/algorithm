#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isBalanced(string num) {
    int diff = 0, sign = 1;
    for (char c : num) {
      int d = c - '0';
      diff += d * sign;
      sign = -sign;
    }
    return diff == 0;
  }
};

int main() {
  vector<tuple<string, bool>> tests {
    {"1234", false},
    {"24123", true},
  };
  
  for (auto &[nums, ans] : tests) {
    assert(Solution().isBalanced(nums) == ans);
  }
}