/*
 * @Date: 2023-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-29
 * @FilePath: /algorithm/cpp/2423_equal_frequency/equal_frequency.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  bool equalFrequency(string word) {
    int charCount[26] = {0};
    for (char c : word) {
      charCount[c - 'a']++;
    }
    unordered_map<int, int> freqCount;
    for (int c : charCount) {
      if (c > 0) {
        freqCount[c]++;
      }
    }
    for (int c : charCount) {
      if (c == 0) {
        continue;
      }
      freqCount[c]--;
      if (freqCount[c] == 0) {
        freqCount.erase(c);
      }
      if (c - 1 > 0) {
        freqCount[c - 1]++;
      }
      if (freqCount.size() == 1) {
        return true;
      }
      if (c - 1 > 0) {
        freqCount[c - 1]--;
        if (freqCount[c - 1] == 0) {
          freqCount.erase(c - 1);
        }
      }
      freqCount[c]++;
    }
    return false;
  }
};

int main() {
  assert(Solution().equalFrequency("abcc") == true);
  assert(Solution().equalFrequency("aazz") == false);
}