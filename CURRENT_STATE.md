## Create a new variable
```java
num = 10;
```

Bytecode representation
```bash
dir global
type int
op +
var num
load num
set 0
add 10
```

## Different datatypes
```java
float 
a = 1.2;
b = 3.4;

int
c = 3;
d = 6783;

bool
e = true;
f = false;
```

Bytecode representation
```bash
dir global
type int
op +
type float
var a
load a
set 0
add 1.2
var b
load b
set 0
add 3.4
type int
var c
load c
set 0
add 3
var d
load d
set 0
add 6783
type bool
var e
load e
set false
get true
var f
load f
set false
get false
```

## If keyword
```java
num1 = 20;
num2 = 30;
num3 = 0;

if 5 < 10 {
    num3 = num1 * num2;
}
```

Bytecode representation
```bash
dir global
type int
op +
var num1
load num1
set 0
add 20
var num2
load num2
set 0
add 30
var num3
load num3
set 0
add 0
block 
con 5
cop <
con 10
skip 
var num3
load num3
set 0
get num1
op *
get num2
end 
```
## While loop which add 1 to num1 until it gets 100
not implemented yet, but would look like this:
```java
num1 = 0;

while num1 < 100 {
  num1 += 1;
}
```

But bytecode interpretation implemented lol
```bash
dir global
type int
op +
var num1
load num1
set 0
block
con num1
cop <
con 100
skip
add 1
jump
end
```
