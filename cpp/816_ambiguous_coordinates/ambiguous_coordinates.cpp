/*
 * @Date: 2022-11-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-07
 * @FilePath: /algorithm/816_ambiguous_coordinates/ambiguous_coordinates.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> getPos(string s) {
    vector<string> pos;
    if (s[0] != '0' || s == "0") pos.push_back(s);
    for (int p = 1; p < s.size(); ++p) {
      if ((p != 1 && s[0] == '0') || s.back() == '0') continue;
      pos.push_back(s.substr(0, p) + "." + s.substr(p));
    }
    return pos;
  }
  vector<string> ambiguousCoordinates(string s) {
    int n = s.size() - 2;
    vector<string> res;
    s = s.substr(1, s.size() - 2);
    for (int l = 1; l < n; ++l) {
      vector<string> lt = getPos(s.substr(0, l));
      if (lt.empty()) continue;
      vector<string> rt = getPos(s.substr(l));
      if (rt.empty()) continue;
      for (auto& i : lt) {
        for (auto& j : rt) {
          res.push_back("(" + i + ", " + j + ")");
        }
      }
    }
    return res;
  }
};

int main() {
  assert(Solution().ambiguousCoordinates("(123)") == vector<string>({"(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"}));
  assert(Solution().ambiguousCoordinates("(00011)") == vector<string>({"(0, 0.011)", "(0.001, 1)"}));
  assert(Solution().ambiguousCoordinates("(0123)") ==
         vector<string>({"(0, 123)", "(0, 1.23)", "(0, 12.3)", "(0.1, 23)", "(0.1, 2.3)", "(0.12, 3)"}));
  assert(Solution().ambiguousCoordinates("(100)") == vector<string>({"(10, 0)"}));
}