# Small projects to learn C++ and Rust.

List from http://www.cplusplus.com/forum/beginner/3473/

## Beginner

1. [x] Write a programme which finds the factorial of a number entered by the user. (check for all conditions).
2. [x] Develop a programme to convert currency X to currency Y and vice versa.
3. [x] Write a programme that prins out a triangle from largest to smallest; user inputs the largest number. Eg:

```
*****
****
***
**
*
```

4. [x] Write a programme that prints out a triangle from smallest to largest; user inputs bottom number. Eg:

```
*
**
***
****
******
```

5. [x] Print out a triangle from smallest to largest, skipping even rows. User inputs largest number, eg:

```
*
***
*****
*******
```

6. [x] Develop a programme that uses a randomly generated number to select 1 of 3 (or more) functions to show the user.
7. [x] Guessing game. ask the user to guess a number between 1 and a 100. If you guessed correctly, it will say you win. If you're too high or too low it will also let you know.
8. [x] Create a programme which generates Fibonacci series til a number 'n', where 'n' is entered by the user. Eg if the user enters 10 then the output would be: `1 1 2 3 5 8`
9. [x] Given a string, determine how many of the characters are vowels and how many are consonants. Terminate the string when the input character encountered is non-alphabetic.
10. [x] Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.
11. [x] List all possible combinations of letters in a 4-letter word. Eg 'TEST' can be unscrambled as TEST, TETS, TSET, TSTE, TTSE, TTES, etc.
12. [x] Make a programme that allows the user to input either the radius, diameter, or area of the circle. The programme should then calculate the other 2 based on the input.
13. [x] Read a line of text and write it out backwards using a recursive function.
14. [x] Write a programme to simulate a simple calculator. It should accept two numbers from the user along with the required operation to be performed. Addition, subtraction, division and multiplication are the basic operations that should be implemented. Feel free to implement other operations. Bonus points for splitting the calculation functions into a separate module.
15. [x] Determine how much money is in a piggy bank that contains several £2 coins, £1 coins, 50p coins, 20p coins, 10p coins and 5p coins. Use the following values to test your programme: one £2, three £1, five 50p  coins, two 20p coins, one 10p coin and fifteen 5p coins.
16. [ ] Create a simple palindrome checker programme. The programme should allow the user to enter a string and check whether the given string is a palindrome or not. Only digits and alphabets should be considered while checking for palindromes - any other characters are to be ignored.
- [ ] Write a programme that allows you to input students' scores and weights. The programme should then calculate a weighted average and score.

## Intermediate

- [ ] Simple file encryption (using something simple like ROT13).
- [ ] Write a programme which will print all the pairs of prime numbers whose sum equals the number entered by the user.
- [ ] Develop a animal classification programme base on the animal kingdom. (for practicing the use of inhabitant classes) ??
- [ ] Write a quiz which retrieves a question and answer from a file. Allow the user to take the quiz, count points total and show score.
- [ ] Write a programme that accepts XHTML, parses and removes the tags, then prints out the remaining text.
- [ ] Write a programme which performs addition, subtraction, multiplication of matrices. The dimensions of both the matrices would be specified by the user (dynamic memory allocation required). Use of structure or a class to define the matrix would be a good idea.
- [ ] Write a programme which will perform the job of moving the file from one location to another. The source and destination path will be entered by the user. Perform the required error checking and handle the exceptions accordingly.
- [ ] Create a sophisticated linked list class. You should be able to insert and delete nodes anywhere in the list, and the nodes should have pointers to nodes both in front and behind them.
- [ ] Create a programme that implements a database. The fields are hard-coded, and the data is saved in a binary file. Although this isn't really flexibility, you aren't relying on any external libraries or functions.
- [ ] Create a command-line todo list. Users should be able to add, complete and delete items. Bonus: use a database (eg SQLite) to persist todo items between programme runs.

## Expert

- [ ] Implement your own `strstr`.
- [ ] Write a programme which acted like a personal planner. A user can input an event, note things to-do on a certain date.
- [ ] Noughts and crosses game.
- [ ] Write a phone/address book programme, with data save in binary files. The users should be able to add/delete/change the data.
- [ ] Write a simple payroll programme, that would include pay rates, and hours work for employees.
- [ ] Create a few classes that model playing cards. Then use this framework to create your favorite card game. Blackjack etc
- [ ] Create a few classes that model chess pieces. Then develop a real chess game.
- [ ] Create a binary tree which has search and sorting functions.
- [ ] Create a Quine, (a programme that prints out its own source code).
- [ ] Implement your own version of the Standard Template Library.

## Graphics

- [ ] Write a programme to draw a rectangle, ellipse, square, circle, point and line based on user input.
- [ ] Write a programme to emulate Microsoft Paint. It should be possible to switch between different tools (circle, rectangle, eraser etc) using pre-defined key strokes.
- [ ] Write a programme to plot a simple x-y graph for a harcoded function (e.g. y=cos(x)). It should be possible to zoom in on any part of the graph.
- [ ] Write a programme to plot a graph of given equation of form y=f(x) and a range for x as command line arguments. (e.g. my_graph_plotter -eq="y=x*x" -xmin=-10, -xmax=10) (PS: more to do with equation solving than graphics)
- [ ] Write the classic brick break-out game (Arkanoid/Breakout)

- [ ] [make a pokedex](http://codereview.stackexchange.com/questions/135293/basic-pokedex-in-c) - use a database, enums for types/gender etc

Good resources for Rust:
- https://github.com/kud1ing/awesome-rust
- https://github.com/ctjhoa/rust-learning
- https://github.com/cis198-2016s
- http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
- http://www.arewewebyet.org/#getting-started
- http://cglab.ca/~abeinges/blah/too-many-lists/book/