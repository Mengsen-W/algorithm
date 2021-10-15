/*
 * @Date: 2021-10-15 09:16:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-15 09:53:45
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string countAndSay(int n) {
    string prev = "1";
    for (int i = 2; i <= n; ++i) {
      string curr = "";
      int start = 0;
      int pos = 0;
      int size = prev.size();
      while (pos < size) {
        while (pos < size && prev[pos] == prev[start]) pos++;
        curr += to_string(pos - start) + prev[start];
        start = pos;
      }
      prev = curr;
    }

    return prev;
  }
};

int main() {
  assert(Solution().countAndSay(1) == "1");
  assert(Solution().countAndSay(4) == "1211");
}