/*
 * @Date: 2022-08-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-03
 * @FilePath: /algorithm/899_orderly_queue/orderly_queue.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string orderlyQueue(string s, int k) {
    if (k == 1) {
      string smallest = s;
      int n = s.size();
      for (int i = 1; i < n; i++) {
        char c = s[0];
        s = s.substr(1);
        s.push_back(c);
        if (s < smallest) {
          smallest = s;
        }
      }
      return smallest;
    } else {
      sort(s.begin(), s.end());
      return s;
    }
  }
};

int main() {
  assert(Solution().orderlyQueue("cba", 1) == "acb");
  assert(Solution().orderlyQueue("baaca", 3) == "aaabc");
}