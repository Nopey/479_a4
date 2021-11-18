//! This file compiles to chfkb and chfkb_dbg
//!

#include "KB.h" // header file of the knowledge base
#include <iostream>
using namespace std;

int main()
{
   cout << "## C++ HFKB (main.cpp)" << endl;

   HF_KB myKB;
   char const *myQuery = "F";

   myKB.TELL("A^B^C", "D");
   myKB.TELL("AB^E", "F");
   myKB.TELL("P1^P2", "B");
   myKB.TELL("", "A");
   myKB.TELL("", "P1");
   myKB.TELL("", "E");
   if (myKB.ASK(myQuery))
      cout << "My knowledge base entails" << myQuery;
   else
      cout << "My knowledge base does not entail " << myQuery;
   cout << endl;
}
