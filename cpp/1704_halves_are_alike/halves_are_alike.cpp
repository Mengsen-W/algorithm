/*
 * @Date: 2022-11-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-11
 * @FilePath: /algorithm/1704_halves_are_alike/halves_are_alike.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool halvesAreAlike(string s) {
    string a = s.substr(0, s.size() / 2);
    string b = s.substr(s.size() / 2);
    string h = "aeiouAEIOU";
    int sum1 = 0, sum2 = 0;
    for (int i = 0; i < a.size(); i++) {
      if (h.find_first_of(a[i]) != string::npos) {
        sum1++;
      }
    }
    for (int i = 0; i < b.size(); i++) {
      if (h.find_first_of(b[i]) != string::npos) {
        sum2++;
      }
    }
    return sum1 == sum2;
  }
};

int main() {
  assert(Solution().halvesAreAlike("book"));
  assert(!Solution().halvesAreAlike("textbook"));
}
