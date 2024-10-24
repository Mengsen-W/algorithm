#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findWinningPlayer(vector<int>& skills, int k) {
    int n = skills.size();
    int cnt = 0;
    int i = 0, last_i = 0;
    while (i < n) {
      int j = i + 1;
      while (j < n && skills[j] < skills[i] && cnt < k) {
        j++;
        cnt++;
      }
      if (cnt == k) {
        return i;
      }
      cnt = 1;
      last_i = i;
      i = j;
    }
    return last_i;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{4, 2, 6, 3, 9}, 2, 2},
      {{2, 5, 4}, 3, 1},
  };

  for (auto &[skills, k, ans] : tests) {
    assert(Solution().findWinningPlayer(skills, k) == ans);
  }
}