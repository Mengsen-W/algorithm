#include <algorithm>
#include <cassert>
#include <numeric>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> survivedRobotsHealths(vector<int>& positions, vector<int>& healths, string directions) {
    int n = positions.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int a, int b) { return positions[a] < positions[b]; });

    vector<tuple<int, int, char>> alive;
    for (int i : idx) {
      int curIdx = i, curHp = healths[i];
      char curDir = directions[i];
      while (!alive.empty()) {
        auto [prevIdx, prevHp, prevDir] = alive.back();
        if (prevDir == 'R' && curDir == 'L') {
          alive.pop_back();
          if (prevHp > curHp) {
            curIdx = prevIdx;
            curHp = prevHp - 1;
            curDir = prevDir;
          } else if (prevHp < curHp) {
            curHp -= 1;
          } else {
            curIdx = -1;
            break;
          }
        } else {
          break;
        }
      }
      if (curIdx != -1) {
        alive.emplace_back(curIdx, curHp, curDir);
      }
    }

    sort(alive.begin(), alive.end());
    vector<int> ans;
    for (auto& [id, hp, dir] : alive) {
      ans.push_back(hp);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, string, vector<int>>> tests{
      {{5, 4, 3, 2, 1}, {2, 17, 9, 15, 10}, "RRRRR", {2, 17, 9, 15, 10}},
      {{3, 5, 2, 6}, {10, 10, 15, 12}, "RLRL", {14}},
      {{1, 2, 5, 6}, {10, 10, 11, 11}, "RLRL", {}},
  };

  for (auto& [positions, healths, directions, expected] : tests) {
    assert(Solution().survivedRobotsHealths(positions, healths, directions) == expected);
  }
}
