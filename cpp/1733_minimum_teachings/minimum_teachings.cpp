#include <cassert>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumTeachings(int n, vector<vector<int>>& languages, vector<vector<int>>& friendships) {
    unordered_set<int> cncon;
    for (auto friendship : friendships) {
      unordered_map<int, int> mp;
      bool conm = false;
      for (int lan : languages[friendship[0] - 1]) {
        mp[lan] = 1;
      }
      for (int lan : languages[friendship[1] - 1]) {
        if (mp[lan]) {
          conm = true;
          break;
        }
      }
      if (!conm) {
        cncon.insert(friendship[0] - 1);
        cncon.insert(friendship[1] - 1);
      }
    }
    int max_cnt = 0;
    vector<int> cnt(n + 1, 0);
    for (auto friendship : cncon) {
      for (int lan : languages[friendship]) {
        cnt[lan]++;
        max_cnt = max(max_cnt, cnt[lan]);
      }
    }
    return cncon.size() - max_cnt;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<vector<int>>, int>> tests{
      {2, {{1}, {2}, {1, 2}}, {{1, 2}, {1, 3}, {2, 3}}, 1},
      {3, {{2}, {1, 3}, {1, 2}, {3}}, {{1, 4}, {1, 2}, {3, 4}, {2, 3}}, 2},
  };

  for (auto [n, languages, friendships, expect] : tests) {
    assert(Solution().minimumTeachings(n, languages, friendships) == expect);
  }
}