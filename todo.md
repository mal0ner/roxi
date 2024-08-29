- add better error reporting (i.e. line numbers) to parse and
  eval commands. probably need to do this by adding line numbers to tokens... somehow. I have probably screwed myself by making it an enum instead of a struct.
- variables, running functions, linking libraries???? holy sheet
- ADD TESTS.
- ADD LIFETIMES INSTEAD OF STRINGS EVERYWHERE.
- use BytePos instead of a simple char index to handle non-ascii characters.
