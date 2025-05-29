#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int dfs(int node, int parent, const vector<vector<int>>& children, int k) {
        if (k < 0) {
            return 0;
        }
        int res = 1;
        for (int child : children[node]) {
            if (child == parent) {
                continue;
            }
            res += dfs(child, node, children, k - 1);
        }
        return res;
    }

    vector<int> build(const vector<vector<int>>& edges, int k) {
        int n = edges.size() + 1;
        vector<vector<int>> children(n);
        for (const auto& edge : edges) {
            children[edge[0]].push_back(edge[1]);
            children[edge[1]].push_back(edge[0]);
        }
        vector<int> res(n);
        for (int i = 0; i < n; i++) {
            res[i] = dfs(i, -1, children, k);
        }
        return res;
    }

    vector<int> maxTargetNodes(vector<vector<int>>& edges1, vector<vector<int>>& edges2, int k) {
        int n = edges1.size() + 1; // int m = edges2.size() + 1;
        vector<int> count1 = build(edges1, k);
        vector<int> count2 = build(edges2, k - 1);
        int maxCount2 = *max_element(count2.begin(), count2.end());
        vector<int> res(n);
        for (int i = 0; i < n; i++) {
            res[i] = count1[i] + maxCount2;
        }
        return res;
    }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, int, vector<int>>> tests{
      {{{0, 1}, {0, 2}, {2, 3}, {2, 4}}, {{0, 1}, {0, 2}, {0, 3}, {2, 7}, {1, 4}, {4, 5}, {4, 6}}, 2, {9, 7, 9, 8, 8}},
      {{{0, 1}, {0, 2}, {0, 3}, {0, 4}}, {{0, 1}, {1, 2}, {2, 3}}, 1, {6, 3, 3, 3, 3}},
  };

  for (auto &[edges1, edges2, k, expected] : tests) {
    assert(Solution().maxTargetNodes(edges1, edges2, k) == expected);
  }
}
