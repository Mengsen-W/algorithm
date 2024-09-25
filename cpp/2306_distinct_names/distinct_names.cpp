#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  long long distinctNames(vector<string>& ideas) {
    unordered_map<char, unordered_set<string>> names;
    for (const string& idea : ideas) {
      names[idea[0]].insert(idea.substr(1, idea.size() - 1));
    }

    auto get_intersect_size = [](const unordered_set<string>& a, const unordered_set<string>& b) -> size_t {
      size_t ans = 0;
      for (const string& s : a) {
        if (b.count(s)) {
          ++ans;
        }
      }
      return ans;
    };

    long long ans = 0;
    for (auto&& [pre_a, set_a] : names) {
      for (auto&& [pre_b, set_b] : names) {
        if (pre_a == pre_b) {
          continue;
        }
        int intersect = get_intersect_size(set_a, set_b);
        ans += static_cast<long long>(set_a.size() - intersect) * (set_b.size() - intersect);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, long long>> tests{
      {{"coffee", "donuts", "time", "toffee"}, 6},
      {{"lack", "back"}, 0},
  };


  for (auto &[ideas, ans] : tests) {
    assert(Solution().distinctNames(ideas) == ans);
  }
}