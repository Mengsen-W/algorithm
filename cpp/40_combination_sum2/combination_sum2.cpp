#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  vector<pair<int, int>> freq;
  vector<vector<int>> ans;
  vector<int> sequence;

 public:
  void dfs(int pos, int rest) {
    if (rest == 0) {
      ans.push_back(sequence);
      return;
    }
    if (pos == freq.size() || rest < freq[pos].first) {
      return;
    }

    dfs(pos + 1, rest);

    int most = min(rest / freq[pos].first, freq[pos].second);
    for (int i = 1; i <= most; ++i) {
      sequence.push_back(freq[pos].first);
      dfs(pos + 1, rest - i * freq[pos].first);
    }
    for (int i = 1; i <= most; ++i) {
      sequence.pop_back();
    }
  }

  vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
    sort(candidates.begin(), candidates.end());
    for (int num : candidates) {
      if (freq.empty() || num != freq.back().first) {
        freq.emplace_back(num, 1);
      } else {
        ++freq.back().second;
      }
    }
    dfs(0, target);
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<vector<int>>>> tests{
      {{10, 1, 2, 7, 6, 1, 5}, 8, {{1, 1, 6}, {1, 2, 5}, {1, 7}, {2, 6}}},
      {{2, 5, 2, 1, 2}, 5, {{1, 2, 2}, {5}}},
  };

  for (auto& [candidates, target, ans] : tests) {
    assert(Solution().combinationSum2(candidates, target) == ans);
  }
}