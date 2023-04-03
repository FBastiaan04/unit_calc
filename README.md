# CLI Unit Calculator in rust
A calculator that understands units
## Usage:
Type "exit" to exit
You can use any unit or combination of units you can think of. Pizza person^-1? Works! No problem at all.
You can define variables. They must start with an alphabetic character, but the rest can be any alphanumeric or underscore.
### Restrictions:
There must be a space before and after every operator.
There must not be a space around ^ when using it to signify the power of a unit.
This means that there is a difference between 3 m ^ 3 and 3 m^3
The first evaluates to 27 m^3 since this and the second to 3 m^3