#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<string> maxNumOfSubstrings(string s) {
    // 记录每个字符的第一次和最后一次出现位置
    unordered_map<char, pair<int, int>> pos;

    for (int i = 0; i < s.length(); i++) {
      char ch = s[i];
      if (pos.find(ch) == pos.end()) {
        pos[ch] = {i, i};
      } else {
        pos[ch].second = i;
      }
    }

    // 所有合法的区间
    vector<pair<int, int>> valid;

    for (auto& [c, range] : pos) {
      int l = range.first, r = range.second;
      int nl = l, nr = l;

      while (nl >= l || nr <= r) {
        int i = (nl >= l) ? nl : nr;

        // 当前处理的是字符 s[i]
        int l_t = pos[s[i]].first;
        int r_t = pos[s[i]].second;

        // 当前区间左侧还有该字符，需要向左扩展
        if (l_t < l) {
          l = l_t;
        }

        // 当前区间右侧还有该字符，需要向右扩展
        if (r_t > r) {
          r = r_t;
        }

        // 当前处理的是左指针
        if (i == nl) {
          nl--;
        }

        // 当前处理的是右指针
        if (i == nr) {
          nr++;
        }
      }

      valid.emplace_back(l, r);
    }

    // 按右端点升序排序
    sort(valid.begin(), valid.end(),
         [](const pair<int, int>& a, const pair<int, int>& b) { return a.second < b.second; });

    // 贪心选择互不重叠的区间
    vector<string> ans;
    int end = -1;

    for (auto& [left, right] : valid) {
      if (left > end) {
        ans.push_back(s.substr(left, right - left + 1));
        end = right;
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<string, vector<string>>> tests{
      {"adefaddaccc", {"e", "f", "ccc"}},
      {"abbaccd", {"d", "bb", "cc"}},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().maxNumOfSubstrings(s) == ans);
  }
}
