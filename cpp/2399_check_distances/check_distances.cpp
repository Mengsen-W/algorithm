/*
 * @Date: 2023-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-09
 * @FilePath: /algorithm/cpp/2399_check_distances/check_distances.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkDistances(string s, vector<int>& distance) {
    vector<int> firstIndex(26);
    for (int i = 0; i < s.size(); i++) {
      int idx = s[i] - 'a';
      if (firstIndex[idx] && i - firstIndex[idx] != distance[idx]) {
        return false;
      }
      firstIndex[idx] = i + 1;
    }
    return true;
  }
};

int main() {
  {
    string s = "abaccb";
    vector<int> distance{1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    bool ans = true;
    assert(Solution().checkDistances(s, distance) == ans);
  }

  {
    string s = "aa";
    vector<int> distance{1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    bool ans = false;
    assert(Solution().checkDistances(s, distance) == ans);
  }
}