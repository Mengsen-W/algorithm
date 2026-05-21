#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
 public:
  int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
    unordered_set<int> seen;
    for (int num : arr1) {
      while (num > 0) {
        seen.insert(num);
        num /= 10;
      }
    }

    int best = 0;
    for (int num : arr2) {
      while (num > 0) {
        if (seen.count(num)) {
          best = max(best, num);
        }
        num /= 10;
      }
    }

    return best == 0 ? 0 : to_string(best).size();
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 10, 100}, {1000}, 3},
      {{1, 2, 3}, {4, 4, 4}, 0},
  };

  for (auto [arr1, arr2, expected] : tests) {
    assert(Solution().longestCommonPrefix(arr1, arr2) == expected);
  }
}
