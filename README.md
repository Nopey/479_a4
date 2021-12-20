# hfkb: Horn Form Knowledge Base
A Horn Form knowledge base.

Although the library is implemented in Rust (see `hfkb.rs`),
I've provided a simple C FFI interface, and C++ wrapper.

The C++ wrapper is consumed in `main.cpp`, which is a minimally modified version of
[the main.cpp file given in the assignment description](http://csci.viu.ca/~liuh/479/assignments/A4.html).


## Compilation
To compile all targets, run `make`.
`make clean` is also supported.

No moving of source code is needed, the directory structure is flat
(with the exception of the doc folder for documentation, which is automatically created).


## Running

To run, simply enter `./chfkb`.
This will run the C++ main, in main.cpp.

There is also `./hfkb`, which will run the Rust main, from main.rs.

As you mention a knowledge base of 200 horn clauses,
I recommend modifying main.cpp to include your preexisting
knowledge base, as both programs use the same Rust module for the actual implementation.


## Inference Rule(s)
In HornFormKb::tell, forward chaining inference rules are applied to simplify the knowledge base.

1. If the clause that is being added implies a variable that is already true, no operation occurs, as that clause has no use.
2. All variable terms in the clause that are already true are simplified (leveraging the boolean algebra rule: B^t=B, where t is true).
3. If the clause we're adding has one or more terms, it is a meaningful clause and can be put into the knowledge base with no additional work.
4. Otherwise, if there are no terms, the clause we're adding is a fact, and we will need to do forward chaining as follows.
5. While there are new facts that we are adding to the knowledge base:
 5.1. Let F be the variable that the fact implies
 5.2. Remove any previous clauses that imply F, and add the fact.
 5.3. For every clause C with term F, remove term F, and..
 5.3.1. If clause C is now a fact (has no terms), add C to the list of facts we are adding to the knowledge base (as used in step 5.)

This inference will make all true variables implied by a single fact,
and runs in O(N), where N is the number of terms in the knowledge base.


## ASK Analysis
HornFormKb::ask calls a HashMap lookup, indexes a vector, and calls Var::is_true().
Var::is_true indexes into a vector and calls HornClause::is_fact.
HornClause::is_fact checks if a vector is empty.

All of these are constant time (although hashmap lookup can be argued to only be near-constant).
Thus, HornFormKb::ask runs in constant time.


## Bugs
While there are no currently known bugs, this code is not to be trusted.


## Comments
The C++ HFKB (chfkb and main.cpp), uses the Rust hfkb.rs module.
It uses it through a C API provided in capi.rs (compiled into libhfkb.a), wrapped up in KB.h's HF_KB C++ class.
