/*
 * @Date: 2021-10-07 18:40:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-07 18:42:59
 * @FilePath: /algorithm/443_count_segments/count_segments.cpp
 * @Description: file content
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int countSegments(string s) {
    int segmentCount = 0;
    int size = s.size();
    for (int i = 0; i < size; i++)
      if ((i == 0 || s[i - 1] == ' ') && s[i] != ' ') segmentCount++;
    return segmentCount;
  }
};

int main() { assert(Solution().countSegments("Hello, my name is John") == 5); }