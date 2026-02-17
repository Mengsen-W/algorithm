#include <cassert>
#include <string>
#include <vector>

using namespace std;

vector<string> readBinaryWatch_enum_HourMinutes(int turnedOn) {
  vector<string> ans;
  for (int h = 0; h < 12; ++h) {
    for (int m = 0; m < 60; ++m) {
      if (__builtin_popcount(h) + __builtin_popcount(m) == turnedOn) {
        ans.push_back(to_string(h) + ":" + (m < 10 ? "0" : "") + to_string(m));
      }
    }
  }
  return ans;
}

vector<string> readBinaryWatch_enum_Binary(int turnedOn) {
  vector<string> ans;
  for (int i = 0; i < 1024; ++i) {
    int h = i >> 6, m = i & 63;  // 用位运算取出高 4 位和低 6 位
    if (h < 12 && m < 60 && __builtin_popcount(i) == turnedOn) {
      ans.push_back(to_string(h) + ":" + (m < 10 ? "0" : "") + to_string(m));
    }
  }
  return ans;
}

int main() {
  assert(readBinaryWatch_enum_Binary(1) == readBinaryWatch_enum_HourMinutes(1));
  assert(readBinaryWatch_enum_Binary(9) == readBinaryWatch_enum_HourMinutes(9));
}
