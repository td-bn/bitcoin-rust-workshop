# [WIP] Rust Bitcoin Workshop

This repo contains a set of exercises that will help the solver gain a better
understanding of Bitcoin core RPCs. It also helps the user gain some
understanding of some of the tools available in the Rust ecosystem, to work
with the Bitcoin network. 

Reading material included in exercises will also help the reader gain a better
understanding of how things work under the hood. It might/might not be needed
to solve the exercises depending on how much you already know.

>NOTE: This is a WIP, currently containing only core RPCs and basic 
transactions. I plan on adding exercises for some more advanced concepts. 

**This workshop is inspired by [this great repo](https://github.com/dtolnay/proc-macro-workshop).**
I found that exercises that build in this manner are great for learning.


## Workflow

Every project has a test suite already written under its tests directory. 
(But feel free to add more tests, remove tests for functionality you don't 
want to implement, or modify tests as you see fit to align with your 
implementation.)

Run `cargo test` inside `basics` directory to run the test suite.

Initially every projects starts with all of its tests disabled. 
Open up the project's tests/progress.rs file and enable tests one at a time 
as you work through the implementation. 

<img width="585" alt="image" src="https://user-images.githubusercontent.com/84708985/213715720-21452bb3-2542-483c-a08b-fe338f824db2.png">

The test files (for example tests/01-setup.rs) each contain a comment 
explaining what functionality is tested and giving some tips for how to 
implement it. I recommend working through tests in numbered order, each time 
enabling one more test and getting it passing before moving on.

<img width="445" alt="image" src="https://user-images.githubusercontent.com/84708985/213716503-96827d9a-5be3-4f13-8f36-6927d9592b95.png">

If a test fails, the test runner will surface the compiler error or 
runtime error output.

<img width="815" alt="image" src="https://user-images.githubusercontent.com/84708985/213716182-cf182f25-0d44-4b9c-a8c3-62818898cf3c.png">


## Debugging tips

The errors give a lot of information about what might be going wrong, but 
on rare occassion I had to re-run the tests and restart the node to make some 
persistent issue go away.

I've added tips where I faced issues while working through this myself.

## Contributing

All contributions are more than welcome. I think exercises like these are the 
best and the fastest way to learn for those of us that learn by doing.

My aim is to make this as useful as possible for those wanting to learn by 
diving straight into the deep end.

I am sure these exercies can be improved and extended. If you see room for 
improvement please create a PR or open an issue so that the improvement can 
be captured/tracked. 

