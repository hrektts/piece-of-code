#include <iostream>
#include <vector>

using namespace std;

class Person {
 protected:
  string firstName;
  string lastName;
  int id;

 public:
  Person(string firstName, string lastName, int identification) {
    this->firstName = firstName;
    this->lastName = lastName;
    this->id = identification;
  }
  void printPerson() {
    cout<< "Name: "<< lastName << ", "<< firstName <<"\nID: "<< id << "\n";
  }
};

class Student : public Person {
 private:
  vector<int> testScores;

 public:
  /*
   *   Class Constructor
   *
   *   Parameters:
   *   firstName - A string denoting the Person's first name.
   *   lastName - A string denoting the Person's last name.
   *   id - An integer denoting the Person's ID number.
   *   scores - An array of integers denoting the Person's test scores.
   */
  Student(string firstName, string lastName, int identification,
          vector<int> testScores)
          : Person(firstName, lastName, identification) {
    this->testScores = testScores;
  }

  /*
   *   Function Name: calculate
   *   Return: A character denoting the grade.
   */
  string calculate() {
    int size = this->testScores.size();
    int sum = 0;
    for (int i = 0; i < size; i++) {
      sum += this->testScores[i];
    }
    int ave = sum / size;
    if (ave < 40) {
      return "T";
    } else if (ave < 55) {
      return "D";
    } else if (ave < 70) {
      return "P";
    } else if (ave < 80) {
      return "A";
    } else if (ave < 90) {
      return "E";
    } else {
      return "O";
    }
  }
};

int main() {
  string firstName;
  string lastName;
  int id;
  int numScores;
  cin >> firstName >> lastName >> id >> numScores;
  vector<int> scores;
  for (int i = 0; i < numScores; i++) {
    int tmpScore;
    cin >> tmpScore;
    scores.push_back(tmpScore);
  }
  Student* s = new Student(firstName, lastName, id, scores);
  s->printPerson();
  cout << "Grade: " << s->calculate() << "\n";
  return 0;
}
