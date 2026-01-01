#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> plusOne(vector<int>& digits) {
    int n = digits.size();
    // 从最低位数开始加1，如果进位则置0，否则
    while (n && ++digits[--n] == 10) digits[n] = 0;
    // 判断最高位
    if (digits[0] == 0) digits.insert(begin(digits), 1);
    return digits;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
    {{1, 2, 3}, {1, 2, 4}},
    {{1, 2, 9}, {1, 3, 0}},
  };

  for (auto& [digits, ans] : tests) {
    assert(Solution().plusOne(digits) == ans);
  }
}
