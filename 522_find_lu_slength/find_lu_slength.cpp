/*
 * @Date: 2022-06-27
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-27
 * @FilePath: /algorithm/522_find_lu_slength/find_lu_slength.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLUSlength(vector<string>& strs) {
    auto is_subseq = [](const string& s, const string& t) -> bool {
      int pt_s = 0, pt_t = 0;
      while (pt_s < s.size() && pt_t < t.size()) {
        if (s[pt_s] == t[pt_t]) {
          ++pt_s;
        }
        ++pt_t;
      }
      return pt_s == s.size();
    };

    int n = strs.size();
    int ans = -1;
    for (int i = 0; i < n; ++i) {
      bool check = true;
      for (int j = 0; j < n; ++j) {
        if (i != j && is_subseq(strs[i], strs[j])) {
          check = false;
          break;
        }
      }
      if (check) {
        ans = max(ans, static_cast<int>(strs[i].size()));
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<string> strs{"aba", "cdc", "eae"};
    assert(Solution().findLUSlength(strs) == 3);
  }

  {
    vector<string> strs{"aaa", "aaa", "aa"};
    assert(Solution().findLUSlength(strs) == -1);
  }
}