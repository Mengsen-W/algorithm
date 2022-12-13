/*
 * @Date: 2022-12-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-13
 * @FilePath: /algorithm/1832_check_if_pangram/check_if_pangram.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkIfPangram(string sentence) {
    int state = 0;
    for (auto c : sentence) {
      state |= 1 << (c - 'a');
    }
    return state == (1 << 26) - 1;
  }
};

int main() {
  assert(Solution().checkIfPangram("thequickbrownfoxjumpsoverthelazydog"));
  assert(!Solution().checkIfPangram("leetcode"));
}