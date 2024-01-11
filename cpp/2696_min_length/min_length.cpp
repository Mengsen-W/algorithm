/*
 * @Date: 2024-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-10
 * @FilePath: /algorithm/cpp/2696_min_length/min_length.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minLength(string s) {
    vector<int> st;
    for (char c : s) {
      st.push_back(c);
      int m = st.size();
      if (m >= 2 && ((st[m - 2] == 'A' && st[m - 1] == 'B') || (st[m - 2] == 'C' && st[m - 1] == 'D'))) {
        st.pop_back();
        st.pop_back();
      }
    }
    return st.size();
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"ABFCACDB", 2},
      {"ACBBD", 5},
  };

  for (auto& [s, ans] : tests) {
    assert(Solution().minLength(s) == ans);
  }
}