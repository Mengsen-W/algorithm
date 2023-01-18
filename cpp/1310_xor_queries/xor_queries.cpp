/*
 * @Date: 2021-05-12 08:57:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-12 09:21:33
 */

#include <cassert>
#include <vector>

using namespace std;

vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
  int n = arr.size();
  vector<int> xors(n + 1);
  for (int i = 0; i < n; i++) {
    xors[i + 1] = xors[i] ^ arr[i];
  }
  int m = queries.size();
  vector<int> ans(m);
  for (int i = 0; i < m; i++) {
    ans[i] = xors[queries[i][0]] ^ xors[queries[i][1] + 1];
  }
  return ans;
}

int main() {
  {
    vector<int> arr{1, 3, 4, 8};
    vector<vector<int>> queries{{0, 1}, {1, 2}, {0, 3}, {3, 3}};
    assert(xorQueries(arr, queries) == std::move(vector<int>{2, 7, 14, 8}));
  }
  {
    vector<int> arr{4, 8, 2, 10};
    vector<vector<int>> queries{{2, 3}, {1, 3}, {0, 0}, {0, 3}};
    assert(xorQueries(arr, queries) == std::move(vector<int>{8, 0, 4, 4}));
  }
}