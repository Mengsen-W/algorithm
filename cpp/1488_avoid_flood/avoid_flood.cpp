#include <cassert>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> avoidFlood(vector<int>& rains) {
    vector<int> ans(rains.size(), 1);
    set<int> st;
    unordered_map<int, int> mp;
    for (size_t i = 0; i < rains.size(); ++i) {
      if (rains[i] == 0) {
        st.insert(i);
      } else {
        ans[i] = -1;
        if (mp.count(rains[i])) {
          auto it = st.lower_bound(mp[rains[i]]);
          if (it == st.end()) {
            return {};
          }
          ans[*it] = rains[i];
          st.erase(it);
        }
        mp[rains[i]] = i;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 2, 3, 4}, {-1, -1, -1, -1}},
      {{1, 2, 0, 0, 2, 1}, {-1, -1, 2, 1, -1, -1}},
      {{1, 2, 0, 1, 2}, {}},
  };

  for (auto& [rains, ans] : tests) {
    assert(Solution().avoidFlood(rains) == ans);
  }
}
