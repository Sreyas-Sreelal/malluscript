# Mallu Script
<p align="center">
  <a href="https://github.com/actions/toolkit"><img alt="GitHub Actions status" src="https://github.com/sreyas-sreelal/malluscript/workflows/build/badge.svg"></a>
</p>

Malluscript is an esoteric scripting language based on manglish and malayalam memes.The language uses trending malayalam memetic words as keywords.The language is not strictly type safe and uses only two datatypes strings and integers.Currently malluscript is on development phase and undergoing vigorous changes.Always check the release section for pre built binaries for interpreter.

## Installation
Either download pre compiled binaries from release page or clone this repository

`git clone https://github.com/sreyas-sreelal/malluscript.git`

## Building
The interpreter can be compiled as follows

```
cd malluscript
cargo build --release
```
Note: You require rust compiler installed on your machine inorder to compile the interpreter

## Executing malluscript programs
Malluscript can be used in two ways
### Interactive shell
To start interactive shell,just run following in terminal

```
./malluscript

```
Something like this would come
```
Mallu Script Version 0.1.0
>> 
```
Here you can start writing your malluscript codes on the way!

### Run malluscript program files
Write the code in file with `.ms` extension and execute it as follows

```
./malluscript file_name.ms
```

## Language Syntax And Grammar

Basic aritmetic operations can be done using `+`,`-`,`*`,`/`,`%` (modulo).Every expression is end with `;`. `{` and `}` introduces new block like c like languages.

### Basic I/O operations
* Printing or writing to console is done as follows
    ```ms
    dhe_pidicho "hello world";
    dhe_pidicho variable;
    dhe_pidicho "Onnum randum kottiyal " + 1+2 + " aane"; 
    ``` 

* In order to get keyboard input from user
  
  For integer inputs
  ```
  variable = number_thada;
  ```
  For string inputs
  ```
  variable = address_thada;
  ```

### Datatypes and Storage
Malluscript is not strictly type safe.The language,currently,supports only integers and string literals as datatypes.

Inorder to declare a variable
```
pwoli_sadhanam variable_name
```
To assign it some value

```
variable_name = 1;
second_var = "ente string";
```

## Conditional Statments And Expressions
The conditional expression has following syintax

```
seriyano i um 0 um same_aane {

} seri_allel {

}
```
The above snippet checks whether i is equals to 0 and if yes the code in first block will execute otherwise block defined by `seri_allel` will get executed.

In general
* `i um 0 um same_aane` checks if i is equals to 0
* `i um 0 um same_alle` checks if i not equals to 0
* `i 0 nekal cheruthane` checks if i is less than 0
* `i 0 nekal veluthane` checks if i is greater than 0

## Iterative Statments Or Loops

The loops in malluscript looks as follows
```
repeat_adi i 0 nekal veluthane {
  i = i-1;
}
```

## Example
A simple program to find factorial in malluscript would be 

```ms
pwoli_sadhanam num;
pwoli_sadhanam factorial;
dhe_pidicho "Input number:";
num = number_thada;
factorial = 1;

repeat_adi 0 um num um same_alle {
    factorial = factorial * num;
    num = num -1;
}

dhe_pidicho "Factoral is : " + factorial + "\n";

```

## Notes
The language as mentioned above is under development phase and it's structure can change overnight,suddenly.This language is not meant to disrespect anyone and wrote just for fun.

Any kind of contribution is always welcome.If you have any ideas or improvements to provide for this project open a pull request or if you have any difficulties using this language open an issue :) 