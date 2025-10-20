#include <cassert>
#include <queue>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  string findLexSmallestString(string s, int a, int b) {
    queue<string> q{{s}};
    unordered_set<string> vis{{s}};
    string ans = s;
    int n = s.size();
    while (!q.empty()) {
      s = q.front();
      q.pop();
      ans = min(ans, s);
      string t1 = s;
      for (int i = 1; i < n; i += 2) {
        t1[i] = (t1[i] - '0' + a) % 10 + '0';
      }
      string t2 = s.substr(n - b) + s.substr(0, n - b);
      for (auto& t : {t1, t2}) {
        if (!vis.count(t)) {
          vis.insert(t);
          q.emplace(t);
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int, int, string>> tests {
    {"5525", 9, 2, "2050"},
    {"74", 5, 1, "24"},
    {"0011", 4, 2, "0011"},
    {"43987654", 7, 3, "00553311"},
  };

  for (auto& [s, a, b, expect] : tests) {
    assert(Solution().findLexSmallestString(s, a, b) == expect);
  }
}