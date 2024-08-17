#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumOperationsToMakeKPeriodic(string word, int k) {
    int n = word.size(), res = INT_MAX;
    unordered_map<string, int> count;
    for (int i = 0; i < n; i += k) {
      string &&part = word.substr(i, k);
      count[part]++;
      res = min(res, n / k - count[part]);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"leetcodeleet", 4, 1},
      {"leetcoleet", 2, 3},
  };

  for (auto &[word, k, ans] : tests) {
    assert(Solution().minimumOperationsToMakeKPeriodic(word,k) == ans);
  }
}