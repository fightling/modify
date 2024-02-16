# Modify

Attach a modified flag to a value and whenever the value is accessed via `get_mut()` this flag will be set until `saved()` is called.

Technical implements `Deref` and `DerefMut` to access the value.

## Example

 ```rs
 use crate::modify::*;

// create new Modify with a 42 in it
let mut value = Modify::new(42);
assert_eq!(value.is_modified(), false);
 
// set the value to 43 and check modified flag
*value = 43;
assert_eq!(value.is_modified(), true);
 
// reset modified flag check modified flag again
value.saved();
assert_eq!(value.is_modified(), false);
```
