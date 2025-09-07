#include <algorithm>
#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> sumZero(int n) {
    vector<int> ans;
    for (int i = 1; i <= n / 2; ++i) {
      ans.push_back(i);
      ans.push_back(-i);
    }
    if (n % 2 == 1) {
      ans.push_back(0);
    }
    return ans;
  }
};

bool check(vector<int>& ans) {
  size_t origin_size = ans.size();
  auto last = std::unique(ans.begin(), ans.end());
  ans.erase(last, ans.end());
  if (ans.size() != origin_size) {
    return false;
  }

  int sum = std::accumulate(ans.begin(), ans.end(), 0);
  if (sum != 0) {
    return false;
  }

  return true;
}

int main() {
  vector<int> tests{5, 3, 1};

  for (auto n : tests) {
    vector<int> ans = Solution().sumZero(n);
    assert(check(ans));
  }
}