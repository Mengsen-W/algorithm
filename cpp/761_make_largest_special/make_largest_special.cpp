/*
 * @Date: 2022-08-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-08
 * @FilePath: /algorithm/761_make_largest_special/make_largest_special.cpp
 */

#include <cassert>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string makeLargestSpecial(string s) {
    if (s.size() <= 2) {
      return s;
    }
    int cnt = 0, left = 0;
    vector<string> subs;
    for (int i = 0; i < s.size(); ++i) {
      if (s[i] == '1') {
        ++cnt;
      } else {
        --cnt;
        if (cnt == 0) {
          subs.push_back("1" + makeLargestSpecial(s.substr(left + 1, i - left - 1)) + "0");
          left = i + 1;
        }
      }
    }

    sort(subs.begin(), subs.end(), greater<string>{});
    string ans = accumulate(subs.begin(), subs.end(), ""s);
    return ans;
  }
};

int main() {
  assert(Solution().makeLargestSpecial("11011000") == "11100100");
  return 0;
}