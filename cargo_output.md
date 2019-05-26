   Compiling fm2gp v0.1.0 (/data/data/com.termux/files/home/projekt/rust-fm2gp)
     Running `rustc --crate-name fm2gp src/lib.rs --color never --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=79b3d95d5f93f62e -C extra-filename=-79b3d95d5f93f62e --out-dir /data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps -C incremental=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental -L dependency=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps`
     Running `rustc --crate-name fm2gp src/lib.rs --color never --emit=dep-info,link -C debuginfo=2 --test -C metadata=fa2b30bc3acfcf38 -C extra-filename=-fa2b30bc3acfcf38 --out-dir /data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps -C incremental=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental -L dependency=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps`
warning: Hard linking files in the incremental compilation cache failed. Copying files instead. Consider moving the cache directory to a file system which supports hard linking in session dir `/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental/fm2gp-2p8y4jm3re97e/s-fcl1gouc32-iswvc5-working`

warning: Hard linking files in the incremental compilation cache failed. Copying files instead. Consider moving the cache directory to a file system which supports hard linking in session dir `/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental/fm2gp-26h6fq1mvszdf/s-fcl1gouf7n-wnrmpr-working`

error[E0382]: use of moved value: `x`
  --> src/lib.rs:44:10
   |
38 |             r = op(r, x);
   |                       - value moved here
...
44 |         x = op(x, x);
   |                ^ value used here after move
   |
   = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `x`
  --> src/lib.rs:44:13
   |
38 |             r = op(r, x);
   |                       - value moved here
...
44 |         x = op(x, x);
   |                   ^ value used here after move
   |
   = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0382`.
error: Could not compile `fm2gp`.

Caused by:
  process didn't exit successfully: `rustc --crate-name fm2gp src/lib.rs --color never --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=79b3d95d5f93f62e -C extra-filename=-79b3d95d5f93f62e --out-dir /data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps -C incremental=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental -L dependency=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error[E0382]: use of moved value: `x`
  --> src/lib.rs:44:10
   |
38 |             r = op(r, x);
   |                       - value moved here
...
44 |         x = op(x, x);
   |                ^ value used here after move
   |
   = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `x`
  --> src/lib.rs:44:13
   |
38 |             r = op(r, x);
   |                       - value moved here
...
44 |         x = op(x, x);
   |                   ^ value used here after move
   |
   = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0382`.
error: Could not compile `fm2gp`.

Caused by:
  process didn't exit successfully: `rustc --crate-name fm2gp src/lib.rs --color never --emit=dep-info,link -C debuginfo=2 --test -C metadata=fa2b30bc3acfcf38 -C extra-filename=-fa2b30bc3acfcf38 --out-dir /data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps -C incremental=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/incremental -L dependency=/data/data/com.termux/files/home/projekt/rust-fm2gp/target/debug/deps` (exit code: 1)
