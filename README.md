# Protolang
This repository contains the compiler for a language called "protolang".
The purpose of this language is to provide basic programming functionality that
should be present in all of the target languages. Complex behavior is not
the purpose of this language, so standard language features may be
missing intentionally due to their incompatability with some of the targets.

## Where is this useful
Imagine that you have a REST API for something like a stock exchange.
Writing the wrapper for this in every language clients may wish to use
is a pain, and means that one language may be updated before another.

With this language you only need to write the logic code once and the code
for all the target languages will be generated.