/*
 * @Date: 2021-08-01 15:01:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-01 15:30:04
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

template <typename T>
class Helper {
  static int partition(vector<T>& nums, int l, int r) {
    T pivot = nums[r];
    int i = l - 1;
    for (int j = l; j <= r - 1; ++j) {
      if (nums[j] <= pivot) {
        i = i + 1;
        swap(nums[i], nums[j]);
      }
    }
    swap(nums[i + 1], nums[r]);
    return i + 1;
  }

  // 基于随机的划分
  static int randomized_partition(vector<T>& nums, int l, int r) {
    int i = rand() % (r - l + 1) + l;
    swap(nums[r], nums[i]);
    return partition(nums, l, r);
  }

  static void randomized_selected(vector<T>& arr, int l, int r, int k) {
    if (l >= r) {
      return;
    }
    int pos = randomized_partition(arr, l, r);
    int num = pos - l + 1;
    if (k == num) {
      return;
    } else if (k < num) {
      randomized_selected(arr, l, pos - 1, k);
    } else {
      randomized_selected(arr, pos + 1, r, k - num);
    }
  }

 public:
  static vector<T> getLeastNumbers(vector<T>& arr, int k) {
    srand((unsigned)time_t(NULL));
    randomized_selected(arr, 0, (int)arr.size() - 1, k);
    vector<T> vec;
    for (int i = 0; i < k; ++i) {
      vec.push_back(arr[i]);
    }
    return vec;
  }
};

class Solution {
 public:
  vector<int> kWeakestRows(vector<vector<int>>& mat, int k) {
    int m = mat.size(), n = mat[0].size();
    vector<pair<int, int>> power;
    for (int i = 0; i < m; ++i) {
      int l = 0, r = n - 1, pos = -1;
      while (l <= r) {
        int mid = (l + r) / 2;
        if (mat[i][mid] == 0) {
          r = mid - 1;
        } else {
          pos = mid;
          l = mid + 1;
        }
      }
      power.emplace_back(pos + 1, i);
    }

    vector<pair<int, int>> minimum =
        Helper<pair<int, int>>::getLeastNumbers(power, k);
    sort(minimum.begin(), minimum.begin() + k);
    vector<int> ans;
    for (int i = 0; i < k; ++i) {
      ans.push_back(minimum[i].second);
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> mat{{1, 1, 0, 0, 0},
                            {1, 1, 1, 1, 0},
                            {1, 0, 0, 0, 0},
                            {1, 1, 0, 0, 0},
                            {1, 1, 1, 1, 1}};
    int k = 3;
    vector<int> ans{2, 0, 3};
    assert(Solution{}.kWeakestRows(mat, k) == ans);
  }
  {
    vector<vector<int>> mat{
        {1, 0, 0, 0}, {1, 1, 1, 1}, {1, 0, 0, 0}, {1, 0, 0, 0}};
    int k = 2;
    vector<int> ans{0, 2};
    assert(Solution{}.kWeakestRows(mat, k) == ans);
  }
  return 0;
}