# walker

AST walking scripting language



## Notes

$ S \rightarrow E $
$ E \rightarrow   E \boxed{+} E \ \vert   \  E \boxed{*} E  \vert \  E \boxed{\div} E  \vert \  E \boxed{-} E  \vert \  ( E ) $ 

Unfortunately this is left recursive so we need to remove that. I use the method outlined [here](https://www.csd.uwo.ca/~mmorenom/CS447/Lectures/Syntax.html/node8.html) to do this:

First we put E in the form $ E \rightarrow E\alpha \vert \beta$

First we factor $E$:
$ S \rightarrow E $

$ E \rightarrow E ( \boxed{+} E \vert \boxed{*} E \vert \boxed{\div} E \vert \boxed{-} E ) | \boxed{(} E \boxed{)} $ 

then you apply the transformation outlined in the linked article 

$S \rightarrow E$
$E \rightarrow \boxed{(} E \boxed{)} E^{\prime} $
$E \rightarrow ( \boxed{+} E \vert \boxed{*} E \vert \boxed{\div} E \vert \boxed{-} E ) E^{\prime} \vert \epsilon $

This grammar is implemented ( i think) and appears to work. Now its just to modify the code to build an AST to evaluate these expressions