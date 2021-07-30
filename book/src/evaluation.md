# Evaluation

Problem Description

> Imagine you need to add a new feature to your existing project, to create image data, then output the image.

How do you approach this problem?

## Discussion

For a few minutes, discuss where do you begin?

* Where do you start?
* What are useful critera to evaluate libraries?
* What is the domain knowledge required here?

## Ways to Approach

Unless there is experience with an existing library, the first step is to find one or a small number of candidates.

Here is a non-exhaustive list of ways:

* check Rust [std](https://doc.rust-lang.org/std/) documentation
* check [crates.io](https://crates.io) & [github](https://github.com) for libraries
* check available [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) & [Are We XX yet](https://arewegameyet.rs/) lists for recommendations
* ask co-workers & friends


## Find Candidates

Once one or a few candidates are found, further evaluation is useful:

* What do you check first?
* What are useful criteria for evaluation?

The following is a non-exhaustive list of things that help assess if a library is suitable.

* check project site
  * a code repository or a dedicated website?
* check library docs.rs documentation
* check README or documentation
  * is there a how to get started section?
  * are there examples?
  * does it provide important information first?
* check activity, issue discussions, number of releases on repository
* do you require additional domain knowledge?
  * do you have to acquire that additional knowledge elsewhere?
  * is that provided by the documentation?
* do tutorials or blog posts exist that describe the library?
* is this library used in other projects?
  * for example in frameworks


## Inspect Candidates

What are the next steps once you decided for a library?

* assess documentation & documentation
* what are important functions / structs / traits etc to know of the library?
* assess the API that you require to build upon
  * high level vs low level
  * what can be ignored, what is necessary?
* one way is to write smallish exploratory examples to get things working
  * helps to get a better understanding
  * shows how complex integration into your project might be
  *
* use [Rust Analyzer]() plugin to figure out API
* inspect code / tests of the library
  * even without much experience it shows how things are related
  * tests can provide minimal programs that show case functionality
* find tutorials or other projects that use the library
  * especially useful for unfamiliar, but more popular libraries
