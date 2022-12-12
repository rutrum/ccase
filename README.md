# `ccase` is a command line utility for converting between string cases

## Usage

### `-t, --to <case>`

Transform a string in**to** a certain string case.

```sh
$ ccase -t snake myVarName
my_var_name
$ ccase -t upper my-var-name
MY VAR NAME
$ ccase -t kebab my_var_name
my-var-name
$ ccase -t pascal "my var name"
MyVarName
```

### `-f, --from <case>`

Splits string based on the boundaries associated with that case.  For example, `snake` case splits on underscores `_` and `camel` splits based on lowercase characters followed by uppercase characters `aA`.

```sh
$ ccase -f camel -t snake myVar-Name
my_var-name
$ ccase -f snake -t snake myVar-name
myvar-name
$ ccase -f kebab -t snake myVar-name
myvar_name
```

### `-b, --boundaries <string>`

Specify precisely what conditions should be used for splitting a string into words.  Whatever boundaries are present in the boundary string will be used against the input.  Any example can do, but a nice way to specify is to separate boundaries with a `:`.  For example, `aA:a1:1a` will split based on lowercase followed by uppercase, lowercase followed by and preceded by a digit.

```sh
$ ccase -b "aA:a1" -t kebab myVar1
my_var_1
$ ccase -b "_:-: " -t camel my_Var-name
my-var-name
$ ccase -b "aAa" -t snake myVar
my_v_ar
$ ccase -b "AAa" -t snake MYVarName # special acronym boundary
my_var_name
```

### `-p, --pattern <pattern>`

Transforms the words of the input based on a pattern.  A pattern describes the order of "word cases".  For example, the camel pattern is a lowercase word followed by capitalized words, like in camel case, while the lower pattern is just lowercased words, like in snake case.

```sh
$ ccase -p camel my-var-name
myVarName
$ ccase -p capital my-var-name
MyVarName
$ ccase -p sentence my-var-name
Myvarname
```

### `-d, --delimeter <string>`

Join the words using a delimeter.  By default, the delimeter is an empty string, as used in camel case.

```sh
$ ccase -p camel -d _ my-var-name
my_Var_Name
$ ccase -p capital -d . my-var-name
My.Var.Name
$ ccase -p sentence -d __ my-var-name
My__var__name
```

## Behavior

# How Case Conversion Works

## List of Cases

| Case           | Example              |
| --             | --                   |
| upper          | UPPER CASE           |
| lower          | lower case           |
| title          | Title Case           |
| toggle         | tOGGLE cASE          |
| camel          | camelCase            |
| pascal         | PascalCase           |
| uppercamel     | UpperCamelCase       |
| snake          | snake_case           |
| uppersnake     | UPPER_SNAKE_CASE     |
| screamingsnake | SCREAMING_SNAKE_CASE |
| kebab          | kebab-case           |
| cobol          | COBOL-CASE           |
| upperkebab     | UPPER-KEBAB-CASE     |
| train          | Train-Case           |
| flat           | flatcase             |
| upperflat      | UPPERFLATCASE        |
| alternating    | aLtErNaTiNg CaSe     |
| random         | RANDOm cAsE          |
| pseudorandom   | PseUdO rAnDoM cASe   |

## List of Patterns

| Pattern     | Definition                    |
| --          | --                            |
| upper       | UPPER, UPPER, ...             |
| lower       | lower, lower, ...             |
| capital     | Capital, Capital, ...         |
| sentence    | Capital, lower, lower, ...    |
| camel       | lower, Capital, Capital, ...  |

## List of Boundaries

| Type       | Example String |
| --         | --             |
| Hyphen     | `-`            |
| Underscore | `_`            |
| Space      | ` `            |
| UpperLower | `Aa`           |
| LowerUpper | `aA`           |
| DigitUpper | `1A`           |
| UpperDigit | `A1`           |
| DigitLower | `1a`           |
| LowerDigit | `a1`           |
| Acronym    | `AAa`          |

# Rust Library

`ccase` is a command line utility built on top of a rust library `convert_case`.  This library and its documentation are the source for the list of cases, patterns, boundaries, and default many behaviors found in this command line utility.
