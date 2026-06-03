#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  int solve(vector<int>& start1, vector<int>& duration1, vector<int>& start2, vector<int>& duration2) {
    int finish1 = INT_MAX;
    for (int i = 0; i < start1.size(); i++) {
      finish1 = min(finish1, start1[i] + duration1[i]);
    }

    int finish2 = INT_MAX;
    for (int i = 0; i < start2.size(); i++) {
      finish2 = min(finish2, max(start2[i], finish1) + duration2[i]);
    }
    return finish2;
  }

 public:
  int earliestFinishTime(vector<int>& landStartTime, vector<int>& landDuration, vector<int>& waterStartTime,
                         vector<int>& waterDuration) {
    int land_water = solve(landStartTime, landDuration, waterStartTime, waterDuration);
    int water_land = solve(waterStartTime, waterDuration, landStartTime, landDuration);
    return min(land_water, water_land);
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>, vector<int>, int>> test_cases = {
      {{2, 8}, {4, 1}, {6}, {3}, 9},
      {{5}, {3}, {1}, {10}, 14},
  };

  for (auto& [landStartTime, landDuration, waterStartTime, waterDuration, expected] : test_cases) {
    assert(Solution().earliestFinishTime(landStartTime, landDuration, waterStartTime, waterDuration) == expected);
  }
}