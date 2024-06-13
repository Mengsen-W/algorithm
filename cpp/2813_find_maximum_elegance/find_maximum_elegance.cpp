#include <cassert>
#include <stack>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  long long findMaximumElegance(vector<vector<int>> &items, int k) {
    sort(items.begin(), items.end(),
         [&](const vector<int> &item1, const vector<int> &item2) -> bool { return item1[0] > item2[0]; });
    unordered_set<int> categorySet;
    long long res = 0, profit = 0;
    stack<int> st;
    for (int i = 0; i < items.size(); i++) {
      if (i < k) {
        profit += items[i][0];
        if (categorySet.count(items[i][1]) == 0) {
          categorySet.insert(items[i][1]);
        } else {
          st.push(items[i][0]);
        }
      } else if (categorySet.count(items[i][1]) == 0 && !st.empty()) {
        profit += items[i][0] - st.top();
        st.pop();
        categorySet.insert(items[i][1]);
      }
      res = max(res, (long long)(profit + categorySet.size() * categorySet.size()));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, long long>> tests{
      {{{3, 2}, {5, 1}, {10, 1}}, 2, 17},
      {{{3, 1}, {3, 1}, {2, 2}, {5, 3}}, 3, 19},
      {{{1, 1}, {2, 1}, {3, 1}}, 3, 7},
  };

  for (auto &[items, k, ans] : tests) {
    assert(Solution().findMaximumElegance(items, k) == ans);
  }
}