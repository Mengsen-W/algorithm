/*
 * @Date: 2022-05-03 07:51:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-03 07:58:23
 * @FilePath: /algorithm/937_reorder_log_files/reorder_log_files.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> reorderLogFiles(vector<string> logs) {
    stable_sort(logs.begin(), logs.end(), [&](const string& log1, const string& log2) {
      int pos1 = log1.find_first_of(" ");
      int pos2 = log2.find_first_of(" ");
      bool isDigit1 = isdigit(log1[pos1 + 1]);
      bool isDigit2 = isdigit(log2[pos2 + 1]);
      if (isDigit1 && isDigit2) {
        return false;
      }
      if (!isDigit1 && !isDigit2) {
        string s1 = log1.substr(pos1);
        string s2 = log2.substr(pos2);
        if (s1 != s2) {
          return s1 < s2;
        }
        return log1 < log2;
      }
      return isDigit1 ? false : true;
    });
    return logs;
  }
};

int main() {
  assert((Solution().reorderLogFiles(
              vector<string>{"dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"}) ==
          vector<string>{"let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"}));
  assert((Solution().reorderLogFiles(
              vector<string>{"a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"}) ==
          vector<string>{"g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"}));
}
