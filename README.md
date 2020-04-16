# Mallu Script
<p align="center">
<img alt="Malluscript" src="images/poster-transparent.png" height=500 width=500><br/>
<a href="https://github.com/actions/toolkit"><img alt="GitHub Actions status" src="https://github.com/sreyas-sreelal/malluscript/workflows/build/badge.svg"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/pulls"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/sreyas-sreelal/malluscript"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/sreyas-sreelal/malluscript"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/blob/master/LICENSE"><img alt="GitHub issues" src="https://img.shields.io/github/license/sreyas-sreelal/malluscript"></a>
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

Basic aritmetic operations can be done using `+`,`-`,`*`,`/`,`%` (modulo).Every expression is end with `;`. `{` and `}` introduces new block like c like languages.Every keywords have alternate ones too,each with different dialects and also in Malayalam unicodes.

### Basic I/O operations
* Printing or writing to console is done as follows
    ```ms
    dhe_pidicho "hello world";
    dhe_pidicho variable;
    dhe_pidicho "Onnum randum kottiyal " + 1+2 + " aane"; 
    ``` 
  * Alternate Keywords
    * `ദേ_പിടിച്ചോ`

* In order to get keyboard input from user
  
  For integer inputs
  ```
  variable = number_thada;
  ```
  * Alternate Keywords
    * `നമ്പർ_താടാ`

  For string inputs
  ```
  variable = address_thada;
  ```
  * Alternate Keywords
    * `അഡ്രസ്_താടാ`

### Datatypes and Storage
Malluscript is not strictly type safe.The language,currently,supports only integers and string literals as datatypes.

Inorder to declare a variable
```
pwoli_sadhanam variable_name
```
  * Alternate Keywords
      * `pwoli_sanam`
      * `pwoli_saanam`
      * `poli_sadhanam`
      * `poli_sanam`
      * `poli_saanam`
      * `പൊളിസാധനം`

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
* `i nekal 0  cheruthane` checks if 0 is less than i
* `i nekal 0  veluthane` checks if 0 is greater than i

  * Alternate Keywords
    * **seriyano**
      * `seriyano_mwone`
      * `seriyano`
      * `ശെരിയാണോ_മോനെ`
    * **cheruthane**
      * `cheruthanenkil`
      * `cheruthanekil`
      * `charuthane`
      * `charuthanenkil`
      * `charuthanekil`
      * `ചെറുതാണെകിൽ`
      * `ചെറുതാണെങ്കിൽ`
    * **same_aane**
      * `സെയിം_ആണേ`
    * **um**
      * `ഉം`
    * **ne_kal**
      * `നെകാൾ`
    * **veluthane**
      * `veluthanenkil`
      * `veluthanekil`
      * `valuthanenkil`
      * `valuthanekil`
      * `valuthane`
      * `വലുതാണെ`
      * `വലുതാണെങ്കിൽ`
      * `വലുതാണെകിൽ`
    * **cheruthane**
      * `cheruthane`
      * `cheruthanenkil`
      * `cheruthanekil`
      * `charuthane`
      * `charuthanenkil`
      * `charuthanekil`
      * `ചെറുതാണെകിൽ`
      * `ചെറുതാണെങ്കിൽ`

## Iterative Statments Or Loops

The loops in malluscript looks as follows
```
repeat_adi 0 nekal i veluthane {
  i = i-1;
}
```
  * Alternative Keywords
    * `റിപീറ്റടി`

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
Same in pure Malayalam would be
```ms
പൊളിസാധനം നമ്പർ;
പൊളിസാധനം ഫാക്ടോറിയൽ;
ദേ_പിടിച്ചോ "ദയവായി നമ്പർ തരുക: ";
നമ്പർ = number_thada;
ഫാക്ടോറിയൽ = 1;

റിപീറ്റടി 0 ഉം നമ്പർ ഉം സെയിം_അല്ല {
    ഫാക്ടോറിയൽ = ഫാക്ടോറിയൽ * നമ്പർ;
    നമ്പർ = നമ്പർ -1;
}

ദേ_പിടിച്ചോ "ഫാക്ടോറിയൽ : " + ഫാക്ടോറിയൽ + "\n";
```

More examples can be found in [examples](examples)

## Notes
The language as mentioned above is under development phase and it's structure can change overnight,suddenly.This language is not meant to disrespect anyone and wrote just for fun.

Any kind of contribution is always welcome.If you have any ideas or improvements to provide for this project open a pull request or if you have any difficulties using this language open an issue :) 