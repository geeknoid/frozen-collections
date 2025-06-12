Returns the key-value pair corresponding to the supplied key.

This is potentially useful:

- for key types where non-identical keys can be considered equal;
- for getting the &K stored key value from a borrowed &Q lookup key; or
- for getting a reference to a key with the same lifetime as the collection.

- The supplied key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.