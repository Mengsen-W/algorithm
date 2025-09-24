#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> replaceElements(vector<int>& arr) {
    int n = arr.size();
    vector<int> ans(n);
    ans[n - 1] = -1;
    for (int i = n - 2; i >= 0; --i) {
      ans[i] = max(ans[i + 1], arr[i + 1]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{17, 18, 5, 4, 6, 1}, {18, 6, 6, 6, 1, -1}},
      {{400}, {-1}},
  };

  for (auto &[arr, ans] : tests) {
    assert(Solution().replaceElements(arr) == ans);
  }
}

