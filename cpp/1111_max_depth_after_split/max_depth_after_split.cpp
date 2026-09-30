#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> maxDepthAfterSplit(string seq) {
    vector<int> ans;
    for (int i = 0; i < (int)seq.size(); ++i) {
      ans.push_back(i & 1 ^ (seq[i] == '('));
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, vector<int>>> tests{
      {"(()())", {0, 1, 1, 1, 1, 0}},
      {"()(())()", {0, 0, 0, 1, 1, 0, 1, 1}},
  };

  for (auto &[seq, ans] : tests) {
    assert(Solution().maxDepthAfterSplit(seq) == ans);
  }
  return 0;
}