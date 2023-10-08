# Contributing to the ⚡ Zom Project
[Contributing to the ⚡ Zom Project]: #contributing-to-the-⚡-zom-project

First or all, thank you considering contribution to the Zom Project!
Any form of contribution is highly appreciated, one of those listed under or simply:
- Star the [Zom Repository]
- Talk about the Zom Programming Language
- Learn the Zom Programming Language
- Create projects using the Zom Programing Language.
- Help other to learn Zom.

## Table of Content

- [Contributing to the ⚡ Zom Project]
  - [License]
  - [Legal Notice]
- [What is the Zom Project?]
- [Code of Conduct]
- [The Zom Programming Language]
  - [Feature Request]
- [Zom Toolchain]
  - [Found a bug ?]
  - [Code contribution]
  - [Doc Contribution]
  - [Review code]

### License
[License]: #license

Licensed under Apache License, Version 2.0 [LICENSE](https://github.com/zom-lang/zom/blob/main/LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0> 
with LLVM-exception <https://foundation.llvm.org/relicensing/LICENSE.txt>

This files may not be copied, modified, or distributed except according to those terms.

> More informations [here](https://github.com/zom-lang/zom/blob/main/NOTICE).

### Legal Notice
[Legal Notice]: #legal-notice

When contributing to the Zom project, you must agree that you have authored 100% of the content, that you have the necessary
rights to the content. And unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you shall be licensed as above, without any additional terms or conditions.

## What is the Zom Project?
[What is the Zom Project?]: #what-is-the-zom-project

The Zom Project is a collection of projects,
* the Zom Programming Language, the actual language.
* the Compiler, the implementation of the Zom Programming Language.
* the tree sitter zom parser (SOON)
* the vscode extension

The Zom Toolchain refer to:
* the Zom Compiler
* tools of the Zom Compiler like,
 - a formatter,
 - a linter,
 - a language server
 - etc...

## Code of Conduct
[Code of Conduct]: #code-of-conduct

This project and everyone participating in it is governed by the [Zom Code of Conduct](/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <zomlang@proton.me>.

## The Zom Programming Language
[The Zom Programming Language]: #the-zom-programming-language

Contribute to the Zom Programming Language.

### Feature Request
[Feature Request]: #feature-request

You found an idea to improve the language ? or the standard library ?

**Before submitting an [issue] for a feature request**
* Do you think your idea fits in the mentality of the project ?
* Do your proposal (or a similar proposal) already has an [issue] ?</br>
  - As it been closed ? Why ? Can I resolve the misconception pointed in the closed [issue] ?
  - If it's open, can I help improving the feature if needed or make proposals on "To Be Determined" (TBD) part of the proposal ?
* Can you, or someone you know, or someone you co-authored the idea with, [can implement the feature into the compiler](#code-contribution) ?
  Because in general a feature request is appreciated if their is already someone who can implement it.

**Submiting an [issue] for a feature request**
* Write a summary, a simple short explication of your idea, with one simple code example if possible.
* Explain in details your idea. Illustrate it with code examples. Introduce new named concept.
* What are alternatives ? Why those alternatives aren't viable?
* What are the drawbacks ? Will it be hard to teach to a new programmer?
* Is their unresolved questions ? What part of the design is not yet determined?

**During the [issue]**
> This part is not really consistent for the moment. In general, a bunch of discussion
> and then your issue is closed or your feature is accepted.
* Discuss with contributor on part of the design that needs to be determined.
* A bunch of discussions, if your idea is a good idea, if it fits in project mentality, etc..
* At some point a contributor may start a poll if the idea is accepted
* If your idea is accepted => [implement your idea](#code-contribution)

## Zom Toolchain
[Zom Toolchain]: #zom-toolchain

Contribute to the Zom Toolchain. Their are multiple ways to contribute to the toolchain:
* [Found a bug ?]
* [Code Contribution]
* [Doc Contribution]
* [Review Code]

### Found a bug ?
[Found a bug ?]: #found-a-bug

You found a bug or an unexpected behavior of the toolchain ? Or your project, doesn't do what it's supposed to do ?

Before creating a new issue, search for a similar issue. Check if it's not your your code that is buggy;
if it is, you can get some help to fix it on the [Discord Server].

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask
you to investigate carefully, collect information and describe the issue in detail in your report.

If it's an
**Internal Compiler Error**
* if possible provide the source code where the compiler crashed,
* a stack trace of the compiler,
* the verbose version, using `zomc -vV`

or an
**Unexpected behavior where it would work**
* if possible provide your project source code,
* a stack trace,
* any error(s) of the OS if their are,
* the OS you are using, its version,
* the verbose version of the toolchain, `zomc -vV`

Then you [can open an issue](https://github.com/zom-lang/zom/issues/new/choose) with your well written bug report.

**After your issue is opened**
* your issue will receive some labels that describes the bug
* a member of the Zom Project will try to recreate your bug or ask for someone to do so,
* if your bug is replicable, we will investigate to know where the error occur and why
* then someone will be assigned to fix the bug or if you can you can contribute and [fix it yourself](#code-contribution)
* then when the bug is fixed, your issue will be closed.

### Code Contribution
[Code Contribution]: #code-contribution

You've seen an issue open where a feature is asked to be implemented ? Or you've seen a feature request
and want to implement it ? Or a bug issue, and want to fix it?

Then you would probably want to contribute to the Zom Codebase.

**Before a code contribution**
* Have you ever program with the Zom Programming Language?
  It is recommanded to before contributing to the compiler.
* Do you have the skills necessary to contribute?
* Do you know Rust, C++ and LLVM? It is mendatory to contribute to make a code contribution.
* Have you read all [the documentation of the compiler](compiler/)?

**First code contribution**
* Fork the [Zom Repository]
* Make sur you have the necessary dependencies, package and tools, then clone your fork to your computer.
* Create a branch, that shortly describe the feature, 
  *e.g:* `feat/goto-statement` -> Feature request of goto statements
         `fix/ice-1234` -> Internal Compiler Error, issue no. 1234
         `fix/out-of-bounds-1234` -> Out of bounds etc etc..
* Make good commits<br/>
  *e.g:* - `update stage1/zom_parser/src/lib.rs`, ❌ -> not enough details<br/>
         - `fix error #NNN + cleanup some code in the parser + fix ICE #NNN + update the email in the README`,
            ❌ -> don't make thousand **different** changes in one commit, make multiples commits.
         - don't make little commits / too many commits, it's useless and more practical to have one commit of
           the same changes instead of multiple little commits.
         - `add parsing for goto statements`, ✅ -> good<br/>
         - `fix typechecking of pointers with tuples #NNNN
            pointers with anonymous structs (tuples), blablalbla
            blabla blabla.. and blabla. Blabla bla bla.`
            ✅ -> very good, don't write a book into commit messages but a good description of the changes is appreciated<br/>
* Push to your fork
* Make sure tests pass.
* Formate your code with `cargo fmt` and fix lints of clippy if their is.
* Open a Pull Request, [PR]

**During the [PR]**
* Your PR will then receive some labels and one or more reviewer(s) (or it will be asked for someone to review your code)
* Your PR will be [reviewed][Review Code]
* Make changes asked by the reviewers and the community
* Then it could be merged:
  - If your PR is a bug fix, the code is well written, your PR would probably quickly be merged
  - If your PR is related to a feature request,
    - If the feature request is accepted, your PR will be merged
    - If the feature request is dismissed, your PR will be closed

### Doc Contribution
[Doc Contribution]: #doc-contribution

You've seen some part of the documentation that are marked as 'TODO' and probably know what would be here? Or
you've seen an [issue] or [PR] asking for the documentation to be ajusted accordingly?

> **TODO** Wrote this part of the contribution guidelines.

### Review code
[Review code]: #review-code

You've seen a [PR] waiting for a reviewer and want to contribute?
Then their is some optional steps that could help you to review code of that [PR].

*This guide asserts you are using the GitHub workflow process.*

**Before a code review**
* Read the 'Before a code contribution' of [Code contribution].

**Review the code**
* Check if the toolchain compiles, clone the fork of the PR and try to compile and run tests.
  If it doesn't compile or pass the tests, you should leave a comment on the PR with your errors.
* Format the code using `cargo fmt` and `clang-format`, no changes would appear, if they are, then request for changes.
* Read the changes he made, is it well programmed?
* Do you have porposal of changes to get the code more cleaner / optimized?
* Do you have questions why the author did that way and not another? Leave a comment on the code.
* Finish your review.

> Note that if your a contributor of the [Zom Repository] your review will probably have more weight, it is always a good
> idea to have at least one or two reviewers that are contributors on your [PR].

[Discord Server]: https://discord.gg/pcDknYP9Bf
[Zom Repository]: https://github.com/zom-lang/zom/
[issue]: https://github.com/zom-lang/zom/issues
[PR]: https://github.com/zom-lang/zom/pulls
