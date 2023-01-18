/*
 * @Date: 2022-06-21 09:53:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:57:15
 * @FilePath: /algorithm/1108_defang_i_paddr/defang_i_paddr.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string defangIPaddr(string address) {
    string ans;
    for (auto& c : address) {
      if (c == '.') {
        ans.append("[.]");
      } else {
        ans.push_back(c);
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().defangIPaddr("1.1.1.1") == "1[.]1[.]1[.]1");
  assert(Solution().defangIPaddr("255.100.50.0") == "255[.]100[.]50[.]0");
}