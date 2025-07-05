#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  char kthCharacter(long long k, vector<int>& operations) {
    int ans = 0;
    k--;
    for (int i = __lg(k); i >= 0; i--) {
      if (k >> i & 1) {
        ans += operations[i];
      }
    }
    return 'a' + (ans % 26);
  }
};

int main() {
  vector<tuple<long long, vector<int>, char>> tests{
      {5, {0, 0, 0}, 'a'},
      {10, {0, 1, 0, 1}, 'b'},
  };

  for (auto &[k, operations, ans] : tests) {
    assert(Solution().kthCharacter(k , operations) == ans);
  }
}