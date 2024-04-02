/*
 * @Date: 2024-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-01
 * @FilePath: /algorithm/cpp/2810_final_string/final_string.cpp
 */

#include <cassert>
#include <deque>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string finalString(string s) {
    deque<char> q;
    bool head = false;
    for (char ch : s) {
      if (ch != 'i') {
        if (head) {
          q.push_front(ch);
        } else {
          q.push_back(ch);
        }
      } else {
        head = !head;
      }
    }
    string ans = (head ? string{q.rbegin(), q.rend()} : string{q.begin(), q.end()});
    return ans;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"string", "rtsng"},
      {"poiinter", "ponter"},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().finalString(s) == ans);
  }
}