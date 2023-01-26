/*
 * @Date: 2023-01-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-26
 * @FilePath: /algorithm/cpp/1663_get_smallest_string/get_smallest_string.cpp
 */

#include <cassert>
#include <string>
using namespace std;

class Solution {
 public:
  string getSmallestString(int n, int k) {
    string ans;
    for (int i = 1; i <= n; i++) {
      int lower = max(1, k - (n - i) * 26);
      k -= lower;
      ans.push_back('a' + lower - 1);
    }
    return ans;
  }
};

int main() {
  {
    int n = 3;
    int k = 27;
    string ans{"aay"};
    assert(Solution().getSmallestString(n, k) == ans);
  }

  {
    int n = 5;
    int k = 73;
    string ans{"aaszz"};
    assert(Solution().getSmallestString(n, k) == ans);
  }
}