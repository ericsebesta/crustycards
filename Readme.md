CrustyCards - a Hearthstone Card interface in Rust

JSON data from https://hearthstonejson.com/

TODO:
process complex field types
- playRequirements
- mechanics
- referencedTags
- entourage
- multiClassGroup

pull in non-collectible cards
start adding queries
- can this class cast this card?
- how many cards are there per set?
- what all sets exist?

define standard sets (add a json for it?)
list all possible sets, error on unknown set (for future error catching)

start breaking up main into files
1 for Card?
1 for loading code (a module?)
