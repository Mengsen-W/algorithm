/*
 * @Date: 2022-04-12 09:11:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-12 09:22:07
 * @FilePath: /algorithm/806_number_of_lines/number_of_lines.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

const int MAX_WIDTH = 100;

class Solution {
 public:
  vector<int> numberOfLines(vector<int> widths, string s) {
    int lines = 1;
    int width = 0;
    for (auto& c : s) {
      int need = widths[c - 'a'];
      width += need;
      if (width > MAX_WIDTH) {
        lines++;
        width = need;
      }
    }
    return {lines, width};
  }
};

int main() {
  assert((Solution().numberOfLines(
              vector<int>{10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                          10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10},
              "abcdefghijklmnopqrstuvwxyz") == vector<int>{3, 60}));

  assert((Solution().numberOfLines(
              vector<int>{4,  10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                          10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10},
              "bbbcccdddaaa") == vector<int>{2, 4}));
}