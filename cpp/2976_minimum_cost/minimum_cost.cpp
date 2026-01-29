#include <cassert>
#include <climits>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    long long minimumCost(string source, string target, vector<char>& original, vector<char>& changed, vector<int>& cost) {
        vector<vector<int>> G(26, vector<int>(26, inf));
        for (int i = 0; i < 26; ++i) {
            G[i][i] = 0;
        }

        int m = original.size();
        for (int i = 0; i < m; ++i) {
            int idx = original[i] - 'a';
            int idy = changed[i] - 'a';
            G[idx][idy] = min(G[idx][idy], cost[i]);
        }

        for (int k = 0; k < 26; ++k) {
            for (int i = 0; i < 26; ++i) {
                for (int j = 0; j < 26; ++j) {
                    G[i][j] = min(G[i][j], G[i][k] + G[k][j]);
                }
            }
        }

        int n = source.size();
        long long ans = 0;
        for (int i = 0; i < n; ++i) {
            int idx = source[i] - 'a';
            int idy = target[i] - 'a';
            if (G[idx][idy] == inf) {
                return -1;
            }
            ans += G[idx][idy];
        }

        return ans;
    }

private:
    static constexpr int inf = INT_MAX / 2;
};

int main() {
  vector<tuple<string, string, vector<char>, vector<char>, vector<int>, long long>> tests{
      {"abcd", "acbe", {'a', 'b', 'c', 'c', 'e', 'd'}, {'b', 'c', 'b', 'e', 'b', 'e'}, {2, 5, 5, 1, 2, 20}, 28},
      {"aaaa", "bbbb", {'a', 'c'}, {'c', 'b'}, {1, 2}, 12},
      {"abcd",  "acbe", {'a'}, {'e'}, {10000}, -1},
  };

  for (auto [source, target, original, changed, cost, ans] : tests) {
    assert(Solution().minimumCost(source, target, original, changed, cost) == ans);
  }

  return 0;
}