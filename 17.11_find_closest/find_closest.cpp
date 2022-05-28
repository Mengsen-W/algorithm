/*
 * @Date: 2022-05-28 10:18:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:22:49
 * @FilePath: /algorithm/17.11_find_closest/find_closest.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int findClosest(vector<string>& words, string word1, string word2) {
    int length = words.size();
    int ans = length;
    int index1 = -1, index2 = -1;
    for (int i = 0; i < length; i++) {
      string word = words[i];
      if (words[i] == word1) {
        index1 = i;
      } else if (words[i] == word2) {
        index2 = i;
      }
      if (index1 >= 0 && index2 >= 0) {
        ans = min(ans, abs(index1 - index2));
      }
    }
    return ans;
  }
};

int main() {
  vector<string> word{"I", "am", "a", "student", "from", "a", "university", "in", "a", "city"};
  string word1{"a"};
  string word2{"student"};
  assert(Solution().findClosest(word, word1, word2) == 1);
}