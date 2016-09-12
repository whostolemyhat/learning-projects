#include <iostream>
#include <string>
#include <sstream>
#include <fstream>
#include <utility>

// https://github.com/nlohmann/json
#include "json.h"

using json = nlohmann::json;

struct Question {
  std::string question;
  std::string answer;
};

std::stringstream open_file(std::string filename) {
  std::ifstream text(filename);
  std::stringstream buffer;
  buffer << text.rdbuf();

  return buffer;
}

std::vector<Question> get_questions(json data) {
  std::vector<Question> questions;
  // for(json::iterator it = data.begin; it != data.end(); it++) {
  for(auto entry : data) {
    Question q;
    q.question = entry["question"];
    q.answer = entry["answer"];
    questions.push_back(q);
  }

  return questions;
}

int main(int argc, char* argv[]) {
  // get filename from args
  if(argc == 1) {
    std::cerr << "Provide a question file" << std::endl;
    return 1;
  }

  std::string filename = argv[1];
  std::stringstream contents = open_file(filename);

  // convert from JSON
  json data = json::parse(contents.str());
  std::vector<Question> questions = get_questions(data);

  int score = 0;
  for(Question q : questions) {
    // std::cout << q.question << "\n" << q.answer << std::endl;
    std::cout << q.question << std::endl;
    std::string answer;
    std::cin >> answer;
    if(!std::cin) {
      std::cerr << "Enter your answer!" << std::endl;
      return 1;
    }

    // transform to lowercase
    std::transform(answer.begin(), answer.end(), answer.begin(), ::tolower);
    std::transform(q.answer.begin(), q.answer.end(), q.answer.begin(), ::tolower);

    if(answer == q.answer) {
      std::cout << "Correct!" << std::endl;
      score++;
    } else {
      std::cout << "Nope" << std::endl;
    }
  }

  std::cout << "You scored " << score << " out of " << questions.size() << std::endl;


  return 0;
}