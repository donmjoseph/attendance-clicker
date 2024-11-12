# Public API endpoints


GET v1/student/answer # gets student answer to a question
GET v1/question/correct # gets correct answer to a question
GET v1/question/student-answers # gets all student answers to a question

POST v1/student/answer
POST v1/question/correct
POST v1/question/create
POST v1/question/delete

one for front end to fetch attendance, poll history, [flashcards stuff]
send poll results to all active 
after poll statistics

in session flags for on going

sync to database from canvas, with classes
homepage front end will poll classes student is enrolled in

calc funcs:
- average attendance after each session
- average marks
