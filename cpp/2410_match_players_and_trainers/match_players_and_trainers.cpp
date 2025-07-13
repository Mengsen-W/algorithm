#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int matchPlayersAndTrainers(vector<int>& players, vector<int>& trainers) {
    sort(players.begin(), players.end());
    sort(trainers.begin(), trainers.end());
    int m = players.size(), n = trainers.size();
    int count = 0;
    for (int i = 0, j = 0; i < m && j < n; i++, j++) {
      while (j < n && players[i] > trainers[j]) {
        j++;
      }
      if (j < n) {
        count++;
      }
    }
    return count;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{4, 7, 9}, {8, 2, 5, 8}, 2},
      {{1, 1, 1}, {10}, 1},
  };

  for (auto& [players, trainers, ans] : tests) {
    assert(Solution().matchPlayersAndTrainers(players, trainers) == ans);
  }
}