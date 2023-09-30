# Contributing to ⚡ Zom

First thank you to consider contributing to the Zom Project!

## License

Licensed under Apache License, Version 2.0 [LICENSE](https://github.com/zom-lang/zom/blob/main/LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0> 
with LLVM-exception <https://foundation.llvm.org/relicensing/LICENSE.txt>

This files may not be copied, modified, or distributed except according to those terms.

> More informations [here](https://github.com/zom-lang/zom/blob/main/NOTICE).

## Legal Notice

When contributing to the Zom project, you must agree that you have authored 100% of the content, that you have the necessary
rights to the content. And unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you shall be licensed as above, without any additional terms or conditions.

## Table of Content

- [Contribute to ⚡ Zom](#contributing-to-⚡-zom)
  - [License](#license)
  - [Legal Notice](#legal-notice)
- [What is the Zom Project ?](#what-is-the-zom-project)
- [Code of Conduct](#code-of-conduct)
- [The Zom Language](#the-zom-language)
- [Zom Compiler](#zom-compiler)
  - [Found a bug ?](#found-a-bug)
  - [Code contribution](#code-contribution)

## What is the Zom Project ?

The Zom Project is a collection of projects,
* the Zom Programming Language, the actual language.
* the Compiler, the implementation of the Zom Programming Language.
* the tree sitter zom parser (SOON)
* the vscode extension (DEPRECATED)

## Code of Conduct

This project and everyone participating in it is governed by the [Zom Code of Conduct](/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <zomlang@proton.me>.

## The Zom Language

Contribute to the Zom Language.

### Feature Request

You found an idea to improve the language ? or standard library ?

**Before submitting an issue for a feature request**
* Do you think your idea fits in the mentality of the project ?
* 

## Zom Compiler

Contribute to the Zom Compiler.

### Found a bug ?

You found a bug or an unexpected behavior of the compiler ? Or your project, doesn't do what it's supposed to do ?

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
* the verbose version of the compiler, `zomc -vV`

Then you [can open an issue](https://github.com/zom-lang/zom/issues/new/choose) with your well written bug report.

**After your issue is opened**
* your issue will receive some labels that describes the bug
* a member of the Zom Project will try to recreate your bug or ask for someone to do so,
* if your bug is replicable, we will investigate to know where the error occur and why
* then someone will be assigned to fix the bug.
* then when the bug is fixed, your issue will be closed.

### Code contribution

You've seen an issue open where a feature is asked to be developed ? Or you've seen a feature request
and want to implement it ? Or a bug issue, and want to fix it?

Then you would probably want to contribute to the Zom Codebase.

Before starting to make a code contribution, please read all [the documentation of the compiler](compiler/).

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
* Open a PR
* Your PR will then receive some labels and one or more reviewer(s) (or it will be asked for someone to review your code)
* Your PR will be reviewed
* Make changes asked by the reviewers and the community
* Then it could be merged:
  - If your PR is a bug fix, the code is well written, your PR would probably quickly be merged
  - If your PR is related to a feature request,
    - If the feature request is accepted, your PR will be merged
    - If the feature request is dismissed, your PR will be closed


[Discord Server]: https://discord.gg/pcDknYP9Bf
[Zom Repository]: https://github.com/zom-lang/zom/