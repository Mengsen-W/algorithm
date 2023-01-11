/*
 * @Date: 2023-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-11
 * @FilePath: /algorithm/2283_digit_count/digit_count.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  bool digitCount(string num) {
    unordered_map<int, int> h;
    int n = num.size();
    for (int i = 0; i < n; i++) {
      h[num[i] - '0']++;
    }
    for (int i = 0; i < n; i++) {
      int v = num[i] - '0';
      if (h[i] != v) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  {
    string num{"1210"};
    bool ans = true;
    assert(Solution().digitCount(num) == ans);
  }

  {
    string num{"030"};
    bool ans = false;
    assert(Solution().digitCount(num) == ans);
  }
}