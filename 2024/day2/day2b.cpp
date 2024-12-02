#include <iostream>
#include <string_view>
#include <vector>
#include <fstream>
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <string_view>
#include <sstream>


using namespace std;

int compare_elements_increasing(int a, int b) {
  return a - b < 0 &&
    abs(a - b) <= 3 &&
    a != b;
}

int compare_elements_decreasing(int a, int b) {
  return a - b > 0 &&
    abs(a - b) <= 3 &&
    a != b;
}

int check_goodness(vector<int> level) {
  if (level.size() <= 2) {
    return 1;
  }
  int skip = 0;
   
  int first = level.at(0);

  int sec = level.at(1);
  int third = level.at(2);
  if (!(compare_elements_decreasing(first, sec) && compare_elements_decreasing(sec, third)) && compare_elements_decreasing(first, third)) {
    level.erase(level.begin() + 1);
    skip++;
  }
  if (!compare_elements_increasing(first, sec) && compare_elements_increasing(first, third)) {
    level.erase(level.begin() + 1);
    skip++;
  }

  if (level.at(0) - level.at(1) > 0) {
    // cout << "lower";
    for (int i = 1; i < level.size(); i++) {
      if (!compare_elements_decreasing(level.at(i-1), level.at(i))) {
        if (skip == 0) {
          cout << "pos: " << i - 1 << " removed: " << level.at(i-1) << ", result: ";
          level.erase(level.begin() + i - 1);
          // cout << "new: " << level.at(i-1) << " " << level.at(i) << '\n';
          skip++;
          i = 0;
          continue;
        } else {
          cout << "0\n";
          return 0;
        }
      }
    }
  } else {
    // cout << "higher";
    for (int i = 1; i < level.size(); i++) {
      if (!compare_elements_increasing(level.at(i-1), level.at(i))) {
        if (skip == 0) {
          cout << "pos: " << i - 1 << " removed: " << level.at(i-1) << ", result: ";
          level.erase(level.begin() + i - 1);
          // cout << "new: " << level.at(i-1) << " " << level.at(i) << '\n';
          skip++;
          i = 0;
          continue;
        } else {
          cout << "0\n";
          return 0;
        }
      }
    }
  }
  cout << "1\n";
  return 1;
}



int main(int argc, char *argv[])
{
  if (argc != 2) {
    cout << "wrong input";
    return 0;
  }
  ifstream f(argv[1]);
  
  string line;

  int total = 0;
  while (getline(f, line)) {
    stringstream ss;
    ss << line;
    string t;
    vector<int> level;
    while (getline(ss, t, ' ')) {
      level.push_back(stoi(t));
    }
    total += check_goodness(level);
  }


  cout << total;
  return 0;
}


