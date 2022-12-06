/*
 * @Date: 2022-12-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-06
 * @FilePath: /algorithm/1805_num_different_integers/num_different_integers.cpp
 */

#include <cassert>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
 public:
  int numDifferentIntegers(string word) {
    unordered_set<string> s;
    int n = word.size(), p1 = 0, p2;
    while (true) {
      while (p1 < n && !isdigit(word[p1])) {
        p1++;
      }
      if (p1 == n) {
        break;
      }
      p2 = p1;
      while (p2 < n && isdigit(word[p2])) {
        p2++;
      }
      while (p2 - p1 > 1 && word[p1] == '0') {  // 去除前导 0
        p1++;
      }
      s.insert(word.substr(p1, p2 - p1));
      p1 = p2;
    }
    return s.size();
  }
};

int main() {
  {
    string word{"a123bc34d8ef34"};
    int ans = 3;
    assert(Solution().numDifferentIntegers(word) == ans);
  }

  {
    string word{"leet1234code234"};
    int ans = 2;
    assert(Solution().numDifferentIntegers(word) == ans);
  }

  {
    string word{"a1b01c001"};
    int ans = 1;
    assert(Solution().numDifferentIntegers(word) == ans);
  }
}