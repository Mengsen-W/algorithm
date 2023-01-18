/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-05 16:10:16
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-05 18:53:21
 */

#include <bits/stdc++.h>

using namespace std;

int in_one_number() {
  int tmp = 0;
  cin >> tmp;
  cout << tmp;
  return 0;
}

int in_vec_with_black() {
  string str;
  vector<int> arr;

  while (cin >> str) {
    arr.push_back(stoi(str));
    if (getchar() == '\n') break;
  }

  for (int i : arr) cout << i << " ";
  cout << endl;

  return 0;
}

vector<int> in_vec_default() {
  // getchar();
  string str = "";
  vector<int> out;
  getline(cin, str);
  regex ws_re(", ");
  vector<string> v(sregex_token_iterator(str.begin(), str.end(), ws_re, -1),
                   sregex_token_iterator());
  transform(v.begin(), v.end(), std::back_inserter(out),
            [](const std::string& s) { return std::stoi(s); });
  // for (auto&& s : out) cout << s << " ";

  return out;
}

int in_vec_with_size() {
  int size = 0;
  cin >> size;
  getchar();
  in_vec_default();
  cout << size;
  return 0;
}

int in_multi_vec_size() {
  int row = 0;  // 行
  int col = 0;  // 列
  cin >> row >> col;
  getchar();
  vector<vector<int>> arr;
  for (int i = 0; i < row; ++i) arr.push_back(in_vec_default());

  for (int i = 0; i < arr.size(); ++i) {
    for (int j = 0; j < arr[i].size(); ++j) {
      cout << arr[i][j] << ", ";
    }
    cout << endl;
  }
  return 0;
}

template <class T>
string VectorToString(vector<T>& vecData) {
  string strData;
  for (auto data : vecData) strData += string(data) + ",";

  strData = strData.substr(0, strData.size() - 1);
  return strData;
}

int main(void) {
  // in_one_number();
  // in_vec_with_black();
  // in_vec_default();
  // in_vec_with_size();
  // in_multi_vec_size();

  vector<string> a{"hello", "world"};
  cout << string(VectorToString<string>(a)) << endl;
  return 0;
}