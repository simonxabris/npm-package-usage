# NPM Package Usage

This is a small CLI tool that tells you how many files import the specified
package and lists those files.

## Installation

You can download the executable from the Releases menu on Github.

## Usage

In the root of your project, run the following command:

`npm-usage <package-name>`

`npm-usage --help` will also give you the above hint.

## Why

This little tool was written for two reasons:

- So I can get better at Rust.
- It can be used to determine whether a project is actually using a dependency or it's just sitting in the package.json for no reason.
