# mluau-require

``mluau-require`` provides a pretty good (spec-wise) require handler for mluau that mostly adheres to Require-By-String specification. For ease of use with VFS's, the following deviations are made:

- Rooted init.luau files are supported
- "/" goes downwards to a "" module 

Currently, mluau-require is based on Lute's implementation of require, ported from C++ to Rust. A update to match the latest lute version is planned.