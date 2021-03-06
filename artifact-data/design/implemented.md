# SPC-data-src
## Loading source code (implementation) links

### [[.load]]: Loading Locations
The process for loading implementation locations is fairly straightforward:
- Define the regular expression of valid names. Valid names inclue:
  - `SRC` and `TST` types ONLY.
  - Any valid postfix name (i.e. `SPC-foo-bar-baz_bob`)
  - (optional) a sub-name specified by a period (i.e. `SPC-foo.sub_impl`).
- Walk the `code_paths`, iterating over each line for the regex and pulling
  out any `Name` or `SubName` locations.

This results in two maps for each file:
- `Name => CodeLoc`
- `SubName => CodeLoc`

## Lints
All lints related to source code are only WARNINGS

- [[.lint_done]]: an artifact with its `done` field set is also linked
  in code.
- [[.lint_exists]]: the artifact name does not exists but it does not specify the
  linked
- [[.lint_subname_exists]]: the artifact name exists but the artifact does not specify
  the linked subname.

### [[.join]]: Joining Locations
The `Name` and `SubName` maps from each file are joined into two large maps
respectively (with any collisions put in the linting vectors which are also
joined).

We must then construct a map of `Name => Implementation` in order for later
steps to construct the full `Artifact` object. We do this by:
- Constructing a map of `Name => Map<SubName, CodeLoc>`, where `Name` is the
  prefix/name of the underlying `SubName`s.
- Building the `Name => Implementation` map by:
  - Draining the `Name => CodeLoc` map and inserting `Implementation` objects.
  - Draining the just created `Name => Map<SubName, CodeLoc>` and either
    modifying or inserting `Implementation` objects.

> Note: we do not worry about whether such `Name` or `SubName`s actually exist.
> That is the job of a later linting step.

# TST-data-src
Only the parsing will be fuzz tested. Joining is covered well enough by a
simple sanity test

- [[.join]]: a simple sanity test suffices. Make sure it covers the following
  cases:
  - name without duplicates
  - name with duplicate in single file
  - name with duplicate in multiple files
  - at least one without primary
  - at least one primary + secondary
  - at least one with multiple secondaries
- [[.parse]]: the sanity test is to simply parse names, both valid and not.
  Expect valid names to be included and invalid names to not be included.
- [[.parse_fuzz]]: there shall be a fuzz test for locations. This is part
  of the larger infrastructure for randomized projects.
