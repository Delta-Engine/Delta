# Delta Programming Language Syntax Reference

This Document shows the Future Plans for the Syntax Rules of Delta.

---

## Variable Declarations

Variables in Delta are declared using the `let` keyword, followed by the variable name, the `be` keyword, and the initial value.

**Syntax:**

```
let <variable_name> be <value>
```

**Examples:**

```delta
let age be 25
let name be "Pranav"
let active be true
let height be 172.5
```

Delta supports several data types, including numbers (integers and floats), strings, booleans, and `nothing` (null).

---

## Constants

Constants are declared using the `const` keyword and cannot be reassigned after initialization.

**Syntax:**

```
const <constant_name> be <value>
```

**Examples:**

```delta
const PI be 3.14159
const MAX_USERS be 100
```

---

## Operators

Delta uses a combination of symbolic and natural language operators to perform operations and comparisons.

### Arithmetic Operators

- `+` : Addition
- `-` : Subtraction
- `*` : Multiplication
- `/` : Division
- `%` : Modulus

**Example:**

```delta
let sum be 5 + 3
let difference be 10 - 2
let product be 4 * 2
let quotient be 20 / 5
let remainder be 10 % 3
```

### Comparison Operators

Delta uses natural language phrases for comparisons:

- `is` : Equal to
- `is not` : Not equal to
- `is greater than` : Greater than
- `is less than` : Less than
- `is greater than or equal` : Greater than or equal to
- `is less than or equal` : Less than or equal to

**Example:**

```delta
when age is greater than 18
    show "Adult"
otherwise
    show "Minor"
```

### Logical Operators

- `and` : Logical AND
- `or` : Logical OR
- `not` : Logical NOT

**Example:**

```delta
when age is greater than 18 and active is true
    show "Active adult"
```

---

## Conditionals

Delta uses the `when` keyword for if-statements and `otherwise` for else clauses.

**Syntax:**

```delta
when <condition>
    <indented block>
otherwise
    <indented block>
```

**Example:**

```delta
when age is greater than or equal 18
    show "You are an adult"
otherwise
    show "You are a minor"
```

Delta also supports switch-like statements using the `choose` keyword, which allows for range-based conditions.

**Syntax:**

```delta
choose <variable>
    when <value1> to <value2>
        <indented block>
    when <value3>
        <indented block>
    otherwise
        <indented block>
```

**Example:**

```delta
choose score
    when 90 to 100
        show "Grade A"
    when 80 to 89
        show "Grade B"
    otherwise
        show "Grade F"
```

---

## Loops

Delta provides several types of loops for different use cases.

### While Loop

**Syntax:**

```delta
repeat while <condition>
    <indented block>
```

**Example:**

```delta
let count be 0
repeat while count is less than 5
    show count
    let count be count + 1
```

### Range-based For Loop

**Syntax:**

```delta
repeat for each number from <start> to <end>
    <indented block>
```

**Example:**

```delta
repeat for each number from 1 to 5
    show number
```

### List Iteration

**Syntax:**

```delta
repeat for each item in <list>
    <indented block>
```

**Example:**

```delta
let colors be ["red", "blue", "green"]
repeat for each color in colors
    show color
```

Delta also supports loop control statements:

- `continue` : Skip to the next iteration
- `break` : Exit the loop

**Example:**

```delta
repeat for each number from 1 to 10
    when number is 5
        continue
    when number is 8
        break
    show number
```

---

## Functions

Functions in Delta are defined using the `define` keyword, followed by the function name, parameters, and the function body.

**Syntax:**

```delta
define <function_name> with <param1>, <param2>, ...
    <indented block>
end
```

**Example:**

```delta
define greet with name
    show "Hello, " + name
end
```

Delta also supports default parameter values.

**Syntax:**

```delta
define <function_name> with <param1>, <param2> be <default_value>
    <indented block>
end
```

**Example:**

```delta
define greet with name, greeting be "Hello"
    show greeting + ", " + name
end
```

Functions can return values using the `return` keyword, and they can return multiple values.

**Example:**

```delta
define get_name_age
    return "Pranav", 25
end

let name, age be get_name_age
```

---

## Input and Output

### Output

Use the `show` keyword to print values to the console.

**Syntax:**

```delta
show <value>
```

**Example:**

```delta
show "Hello, world"
show name
```

### Input

Use the `ask` keyword to get string input from the user, and `ask number` for numeric input.

**Syntax:**

```delta
let <variable> be ask "<prompt>"
let <variable> be ask number "<prompt>"
```

**Example:**

```delta
let user_input be ask "Enter your name: "
let age be ask number "Enter your age: "
```

---

## Strings

Strings in Delta are enclosed in double quotes and support concatenation using the `+` operator.

**Example:**

```delta
let full_name be "Pranav" + " Verma"
show "Hello, " + full_name
```

Delta also supports string interpolation using curly braces `{}`.

**Example:**

```delta
show "My name is {name} and I am {age} years old"
```

Additionally, Delta provides string operations like `contains`, `starts with`, and `ends with`.

**Example:**

```delta
when "hello" contains "ell"
    show "Found substring"
```

---

## Lists

Lists in Delta are ordered collections of items, declared using square brackets `[]`.

**Syntax:**

```delta
let <list_name> be [<item1>, <item2>, ...]
```

**Example:**

```delta
let colors be ["red", "blue", "green"]
```

Delta provides several operations for lists:

- `add <item> to <list>` : Append an item to the list
- `remove <item> from <list>` : Remove an item from the list
- `length of <list>` : Get the number of items in the list
- `first of <list>` : Get the first item
- `last of <list>` : Get the last item

**Example:**

```delta
add "yellow" to colors
remove "red" from colors
show length of colors
show first of colors
```

Delta also supports range creation for lists.

**Example:**

```delta
let numbers be 1 to 10  // Creates [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

---

## Objects

Objects in Delta are key-value pairs, similar to dictionaries or structs in other languages.

**Syntax:**

```delta
let <object_name> be {
    <property1>: <value1>,
    <property2>: <value2>
}
```

**Example:**

```delta
let user be {
    name: "Pranav",
    age: 25
}
```

Properties can be accessed using dot notation.

**Example:**

```delta
show user.name
let user.age be 26
```

---

## Error Handling

Delta uses `attempt` and `rescue` for error handling, similar to try-catch in other languages.

**Syntax:**

```delta
attempt
    <indented block>
rescue error
    <indented block>
```

**Example:**

```delta
attempt
    let result be divide 10, 0
rescue error
    show "Cannot divide by zero: " + error
```

---

## Module System

Delta supports importing modules to use functions and variables defined in other files or built-in libraries.

**Syntax:**

```delta
import "<module_name>"
import "<module_name>" as <alias>
import <function1>, <function2> from "<module_name>"
```

**Examples:**

```delta
import "math"
let result be math.calculate_square_root 16

import "user_manager" as users
let new_user be users.create_user "Pranav", 25

import square_root, power from "math"
let result be square_root 25
```

Delta's module resolution order is:

1. Local files (`.de` files in the current directory)
2. Built-in libraries (embedded in the executable)
3. System-wide modules (if applicable)

---

## Built-in Libraries

Delta provides several built-in libraries that can be imported to extend the language's functionality. Below is a list of available built-in libraries and their key functions.

### File System (`file_system`)

- `read_file "<filename>"` : Read the contents of a file
- `write_file "<filename>", <content>` : Write content to a file
- `delete_file "<filename>"` : Delete a file
- `file_exists "<filename>"` : Check if a file exists
- `create_directory "<dirname>"` : Create a new directory
- `list_directory "<dirname>"` : List the contents of a directory

**Example:**

```delta
import "file_system"
let content be file_system.read_file "data.txt"
file_system.write_file "output.txt", content
```

### Date and Time (`date_time`)

- `now` : Get the current date and time
- `format_time <time>, "<format>"` : Format a time value
- `parse_time "<time_str>", "<format>"` : Parse a time string
- `add_days <time>, <days>` : Add days to a time value
- `subtract_days <time>, <days>` : Subtract days from a time value

**Example:**

```delta
import "date_time"
let current_time be date_time.now
let formatted_time be date_time.format_time current_time, "YYYY-MM-DD"
```

### String Utilities (`string_utils`)

- `to_upper <string>` : Convert string to uppercase
- `to_lower <string>` : Convert string to lowercase
- `trim <string>` : Remove leading and trailing whitespace
- `split <string>, <delimiter>` : Split string into a list
- `join <list>, <delimiter>` : Join list elements into a string
- `replace <string>, <old>, <new>` : Replace substrings

**Example:**

```delta
import "string_utils"
let clean_text be string_utils.trim "  hello world  "
let words be string_utils.split clean_text, " "
```

### Math (`math`)

- `square_root <number>` : Calculate the square root
- `power <base>, <exponent>` : Raise base to the exponent
- `absolute <number>` : Get the absolute value
- `round <number>` : Round to the nearest integer
- `floor <number>` : Round down to the nearest integer
- `ceiling <number>` : Round up to the nearest integer
- `random <min>, <max>` : Generate a random number between min and max

**Example:**

```delta
import "math"
let result be math.power 2, 3
let random_num be math.random 1, 100
```

### Network (`network`)

- `http_get "<url>"` : Perform an HTTP GET request
- `http_post "<url>", <data>` : Perform an HTTP POST request
- `download_file "<url>", "<filename>"` : Download a file from a URL
- `upload_file "<filename>", "<url>"` : Upload a file to a URL

**Example:**

```delta
import "network"
let response be network.http_get "https://api.example.com/data"
```

### System (`system`)

- `run_command "<command>"` : Run a shell command
- `get_environment "<var>"` : Get the value of an environment variable
- `set_environment "<var>", "<value>"` : Set an environment variable
- `exit_program <code>` : Exit the program with a status code

**Example:**

```delta
import "system"
let output be system.run_command "ls -l"
```

### JSON (`json`)

- `parse_json "<json_str>"` : Parse a JSON string into an object
- `to_json <object>` : Convert an object to a JSON string
- `validate_json "<json_str>"` : Check if a string is valid JSON

**Example:**

```delta
import "json"
let data be json.parse_json "{\"name\": \"Pranav\"}"
let json_str be json.to_json data
```

---

## Reserved Keywords

The following keywords are reserved in Delta and cannot be used as identifiers:

`let`, `be`, `const`, `when`, `otherwise`, `choose`, `repeat`, `while`, `for`, `each`, `from`, `to`, `in`, `define`, `with`, `end`, `return`, `show`, `ask`, `number`, `continue`, `break`, `is`, `not`, `and`, `or`, `greater`, `than`, `less`, `equal`, `contains`, `starts`, `ends`, `string`, `boolean`, `list`, `object`, `true`, `false`, `nothing`, `add`, `remove`, `length`, `first`, `last`, `of`, `attempt`, `rescue`, `error`, `import`, `as`

---

This reference covers all the essential syntax rules and features of the Delta programming language, providing a detailed guide for developers.