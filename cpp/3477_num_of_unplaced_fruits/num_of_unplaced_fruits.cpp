#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int numOfUnplacedFruits(vector<int>& fruits, vector<int>& baskets) {
        int count = 0;
        int n = baskets.size();
        for (auto fruit : fruits) {
            int unset = 1;
            for (int i = 0; i < n; i++) {
                if (fruit <= baskets[i]) {
                    baskets[i] = 0;
                    unset = 0;
                    break;
                }
            }
            count += unset;
        }
        return count;
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests {
    {{4,2,5}, {3,5,4}, 1},
    {{3,6,1}, {6,4,7}, 0},
  };

  for (auto& [fruits, baskets, expect] : tests) {
    assert(Solution().numOfUnplacedFruits(fruits, baskets) == expect);
  }
}