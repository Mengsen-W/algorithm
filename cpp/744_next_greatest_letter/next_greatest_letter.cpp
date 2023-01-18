/*
 * @Date: 2022-04-02 23:34:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-02 23:42:37
 * @FilePath: /algorithm/744_next_greatest_letter/next_greatest_letter.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  char nextGreatestLetter(vector<char> letters, char target) {
    return target < letters.back()
               ? *upper_bound(letters.begin(), letters.end() - 1, target)
               : letters[0];
  }
};

int main() {
  assert(Solution().nextGreatestLetter({'c', 'f', 'j'}, 'a') == 'c');
  assert(Solution().nextGreatestLetter({'c', 'f', 'j'}, 'c') == 'f');
  assert(Solution().nextGreatestLetter({'c', 'f', 'd'}, 'c') == 'f');
}