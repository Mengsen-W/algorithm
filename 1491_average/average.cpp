/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:02:04
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:35:09
 */

#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

using namespace std;

double average(vector<int>& salary) {
  double maxValue = *max_element(salary.begin(), salary.end());
  double minValue = *min_element(salary.begin(), salary.end());
  double sum = accumulate(salary.begin(), salary.end(), -maxValue - minValue);
  return sum / int(salary.size() - 2);
}

int main(void) {
  vector<int> salary;
  salary = {4000, 3000, 1000, 2000};
  cout << average(salary) << endl;
  salary = {1000, 2000, 3000};
  cout << average(salary) << endl;
  salary = {6000, 5000, 4000, 3000, 2000, 1000};
  cout << average(salary) << endl;
  salary = {8000, 9000, 2000, 3000, 6000, 1000};
  cout << average(salary) << endl;
}