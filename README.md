VecGrow
===

It is similar to a regular `Vec`, with one exception - 
the size is not reduced. `VecGrow` can constantly grow, 
but not decrease, and when objects are deleted, 
their place remains for the new object as a free index, 
which eases the memory power in the case of permanent 
deletion and creation of objects.

## Examples
``` rust
use vector_growing::*;

let mut vg = VecGrow::new();
vg.push(1);
vg.push(2);

assert_eq!(vg[0], Some(1));
assert_eq!(vg[1], Some(2));

vg.remove(0);

assert_eq!(vg[0], None);

vg.push(1);

assert_eq!(vg[0], Some(1));
```
Initializing VecGrow using a macro:
``` rust
use vector_growing::*;

let vg_empty: VecGrow<u8> = vec_grow![];
let vg_num = vec_grow![1, 2, 3];
let vg_zero = vec_grow![0; 100];

assert!(vg_empty.is_empty());
assert_eq!(vg_num[1], Some(2));
assert_eq!(vg_zero[99], Some(0));
```