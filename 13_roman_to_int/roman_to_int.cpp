/*
 * @Date: 2021-05-15 14:06:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-15 14:10:43
 */

#include <cassert>
#include <string>
#include <unordered_map>
using namespace std;

unordered_map<char, int> symbolValues = {
    {'I', 1},   {'V', 5},   {'X', 10},   {'L', 50},
    {'C', 100}, {'D', 500}, {'M', 1000},
};

int romanToInt(string s) {
  int ans = 0;
  int n = s.length();
  for (int i = 0; i < n; ++i) {
    int value = symbolValues[s[i]];
    if (i < n - 1 && value < symbolValues[s[i + 1]]) {
      ans -= value;
    } else {
      ans += value;
    }
  }
  return ans;
}

int main() {
  assert(romanToInt("III") == 3);
  assert(romanToInt("IV") == 4);
  assert(romanToInt("IX") == 9);
  assert(romanToInt("LVIII") == 58);
  assert(romanToInt("MCMXCIV") == 1994);
  return 0;
}
