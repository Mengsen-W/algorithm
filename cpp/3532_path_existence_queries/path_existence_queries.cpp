#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<bool> pathExistenceQueries(int n, vector<int>& nums, int maxDiff, vector<vector<int>>& queries) {
    vector<int> rights;
    for (int i = 1; i < n; i++) {
      if (nums[i] - nums[i - 1] > maxDiff) {
        rights.push_back(i - 1);
      }
    }
    rights.push_back(n - 1);

    vector<bool> res(queries.size());
    for (int i = 0; i < queries.size(); i++) {
      int x = queries[i][0];
      int y = queries[i][1];

      res[i] = ranges::lower_bound(rights, x) == ranges::lower_bound(rights, y);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<int>, int, vector<vector<int>>, vector<bool>>> tests{
      {2, {1, 3}, 1, {{0, 0}, {0, 1}}, {true, false}},
      {4, {2, 5, 6, 8}, 2, {{0, 1}, {0, 2}, {1, 3}, {2, 3}}, {false, false, true, true}},
  };

  for (auto& [n, nums, maxDiff, queries, ans] : tests) {
    assert(Solution().pathExistenceQueries(n, nums, maxDiff, queries) == ans);
  }
}
