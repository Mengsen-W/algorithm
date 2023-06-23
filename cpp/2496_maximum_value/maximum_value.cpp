/*
 * @Date: 2023-06-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-23
 * @FilePath: /algorithm/cpp/2496_maximum_value/maximum_value.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumValue(vector<string>& strs) {
    auto f = [](string& s) {
      int x = 0;
      for (char& c : s) {
        if (!isdigit(c)) {
          return (int)s.size();
        }
        x = x * 10 + c - '0';
      }
      return x;
    };
    int ans = 0;
    for (auto& s : strs) {
      ans = max(ans, f(s));
    }
    return ans;
  }
};

int main() {
  {
    vector<string> strs{"alic3", "bob", "3", "4", "00000"};
    int ans = 5;
    assert(Solution().maximumValue(strs) == ans);
  }

  {
    vector<string> strs{"1", "01", "001", "0001"};
    int ans = 1;
    assert(Solution().maximumValue(strs) == ans);
  }
}