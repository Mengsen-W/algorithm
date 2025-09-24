#include <cassert>
#include <tuple>
#include <vector>

using namespace  std;

class Solution {
public:
    bool threeConsecutiveOdds(vector<int>& arr) {
        int n = arr.size();
        for (int i = 0; i <= n - 3; ++i) {
            if ((arr[i] & 1) & (arr[i + 1] & 1) & (arr[i + 2] & 1)) {
                return true;
            }
        }
        return false;
    }
};

int main() {
  vector<tuple<vector<int>, bool>> tests {
		{{2, 6, 4, 1}, false},
		{{1, 2, 34, 3, 4, 5, 7, 23, 12}, true},  };

    for (auto &[arr, ans] : tests) {
      assert(Solution().threeConsecutiveOdds(arr) == ans);
    }
}