GITHUB: https://github.com/hkunkel2/lolcode

Tested on MAC, but used Open crate so should work with either.
Tested with google chrome as default browser, but should work with any.

I started using AI after converting the code from lab 4 to the project 1 grammer since the solution was already given.
I used claude code in plan mode for bounce ideas off and refine the ideas in order to create a plan.  
I did this for generating the syntax tree, handling static semantic errors, resolving scope and replacing variable references with values, and generating the html.
Then review the code as claude is generating it, accepting and adjusting until code is complete.  Then we test and repeat for an issue.
I also had claude generate more test files, based off of given scenarios.

* Note when trying to sumbit transcripts, issue with vscode turncating terminal output and losing coversions and output... 
Also could be that I use /clear between tasks to keep control of context to claude, habit I have from work


Usage: 
bin/lolcompiler <input_file>