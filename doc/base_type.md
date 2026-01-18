## Use Macro genentor Vector

use macro generate vector method can get parameter by name, e.g.
```rust
let a = Vec3::new(1.0, 1.0, 1.0);
println!("vec {} {} {}", a.x, a.y, a.z);
```

## generic matrix type
generic matrix can generate matrix by matrix producot,
$ A_{ij} \otimes B_{jk} = C_{ik} $
e.g.
```rust
let a = Mat<f32, 4, 3>::default();
let b = Mat<f32, 3, 12>::default();

println!("{:?}", a * b);
```
