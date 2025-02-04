# cmprs Changelog

This document tracks the changes between cmprs versions. Dates are written in the MM/DD/YYYY format.

## 1.1.0 (Unreleased)

### Additions

- Added support for multiple input paths: `cmprs file1 file2 ...`.
  - This allows cmprs to work with glob expansion in shells like Bash and Zsh. `cmprs dir/*` can be used instead of `cmprs "dir/*"`.
- Added support for custom formats of output files with the `-o`, `--output-format` option.

### Improvements

- Limited all displays of file size to 2 decimal points.
- Added an informational message when the set of input files is empty.
- Improved display of file I/O errors.

### Removals

- The shortened `-o` option for `--overwrite` has been removed. `-o` is now short for `--output-format`.

## 1.0.0 (02/01/2025)

Initial release of cmprs.
