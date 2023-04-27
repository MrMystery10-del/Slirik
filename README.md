# Slirik
Slirik is a modern programming language designed to offer the best of both worlds: the simplicity and ease of use of a scripting language, as well as the power and performance of a deep language. Slirik has a unique mix of a compiler and interpreter, making it flexible and efficient.

# IMPORTANT
Slirik is under developement, not all features on that readme are already implemented
Read this for more information on current progress https://github.com/MrMystery10-del/Slirik/blob/main/CURRENT_STATE.md

## Features
- Slirik has a simple and intuitive syntax, making it easy for beginners to learn and use.
- It is a compiled language, which means that code written in Slirik can be executed faster than interpreted languages like Python.
- Slirik supports dynamic typing, making it easy to write and debug code.
- It also supports static typing, allowing for increased code optimization and error detection.
- Slirik is a strongly typed language, which ensures type safety and helps catch errors early in the development process.
- It has built-in memory management, which makes it easier to avoid memory leaks and other common programming errors.

## Getting Started
To get started with Slirik, you will need to install the compiler and interpreter. You can find the latest version of the compiler and interpreter on our GitHub page.

Once you have installed Slirik, you can start writing code using your favorite text editor. Slirik files have the extension .sk

## Examples
Here is an example of x to the power of y function written in Slirik:
```java
function main(args | String[]) {
  int x = 5;
  int y = 3;
  
  println!("The result of {} to the power of {} is: {}!", x, y, pow(x, y));
}

function pow(x, y | int) &int {
  result = x;
  for 1..y {
    result *= x;
  }
  return result;
}
```

You can also use methods inside of your functions, here is the same example but with using methods:
```java
function main(args | String[]) {
  int x = 5;
  int y = 3;
  
  println!("The result of {} to the power of {} is: {}!", x, y, pow());
  
  pow() &int {
    result = x;
    for 1..y {
      result *= x;
    }
    return result;
  }
}
```

Output of both examples
```bash
The result of 5 to the power of 3 is: 125!
```

## Contributing
We welcome contributions to Slirik from the community. If you would like to contribute, please read our contributing guidelines to get started.

## License
Slirik is released under the Apache 2.0 License.
