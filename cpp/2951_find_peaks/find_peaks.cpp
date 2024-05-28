#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findPeaks(vector<int> &mountain) {
    vector<int> res;
    for (int i = 1; i + 1 < mountain.size(); i++) {
      if (mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1]) {
        res.push_back(i);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{2, 4, 4}, {}},
      {{1, 4, 3, 8, 5}, {1, 3}},
  };

  for (auto &[mountain, ans] : tests) {
    assert(Solution().findPeaks(mountain) == ans);
  }
}