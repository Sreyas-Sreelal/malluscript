# Malluscript
<p align="center">
<img alt="Malluscript" src="images/poster-transparent.png" height=500 width=500><br/>
<a href="https://github.com/actions/toolkit"><img alt="GitHub Actions status" src="https://github.com/sreyas-sreelal/malluscript/workflows/build/badge.svg"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/pulls"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/sreyas-sreelal/malluscript"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/sreyas-sreelal/malluscript"></a>
<a href="https://github.com/Sreyas-Sreelal/malluscript/blob/master/LICENSE"><img alt="GitHub issues" src="https://img.shields.io/github/license/sreyas-sreelal/malluscript"></a>
<p align="center">
<a href="https://patreon.com/sreyas_sreelal"><img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dsreyas_sreelal%26type%3Dpatrons&style=for-the-badge" /> </a>
<p align="center">
<a href='https://ko-fi.com/sreyas' target='_blank'><img height='40'  src='https://az743702.vo.msecnd.net/cdn/kofi3.png?v=0' border='0' alt='Buy Me a Coffee at ko-fi.com' />
</p></p>
<p align="center">
<a href="https://discord.gg/9GDUE4hRcW"><img alt="Malluscript" src="https://dcbadge.vercel.app/api/server/9GDUE4hRcW"></a>
</p></p>


Malluscript is an esoteric scripting language that allows one to write computer programs in Malayalam. The concept of Malluscript stems from the idea of promoting inclusivity and linguistic diversity within the world of coding. By enabling programmers to express their ideas and algorithms in Malayalam, Malluscript serves as a bridge between traditional programming languages and the rich linguistic heritage of the Malayalam-speaking community. Currently, Malluscript is on the development phase and undergoing vigorous changes. Always check the release section for pre-built binaries for the interpreter.

മലയാളത്തിൽ കമ്പ്യൂട്ടർ പ്രോഗ്രാമുകൾ എഴുതാൻ സഹായിക്കുന്ന ഒരു സ്ക്രിപ്റ്റിംഗ് ഭാഷയാണ് മല്ലുസ്ക്രിപ്റ്റ്. കോഡിംഗിൽ പ്രാദേശിക ഭാഷകളുടെ ഉപയോഗം പ്രോത്സാഹിപ്പിക്കുക എന്ന ലക്ഷ്യത്തോടെയാണ് മല്ലുസ്ക്രിപ്റ്റ് രൂപകൽപ്പന ചെയ്തിരിക്കുന്നത്. പ്രോഗ്രാമർമാർക്ക് അവരുടെ ആശയങ്ങളും അൽഗോരിതങ്ങളും സ്വന്തം ഭാഷയിൽ പ്രകടിപ്പിക്കാൻ അനുവദിക്കുന്നതിലൂടെ ഇത് പരമ്പരാഗത പ്രോഗ്രാമിംഗ് ഭാഷകൾക്കും മലയാളത്തിനും ഇടയിലുള്ള ഒരു പാലമായി പ്രവർത്തിക്കുന്നു. നിലവിൽ മല്ലുസ്ക്രിപ്റ്റ് വികസനത്തിന്റെ ഘട്ടത്തിലാണ്. ഇൻ്റർപ്രെറ്ററിനായി റിലീസ് പേജിൽ നിന്ന് കംപൈൽ ചെയ്ത ബൈനറികൾ ഡൗൺലോഡ് ചെയ്യാവുന്നതാണ്.

**Note:** For support please join our [Discord](https://discord.gg/9GDUE4hRcW). / **ശ്രദ്ധിക്കുക:** സഹായത്തിനായി ഞങ്ങളുടെ [Discord](https://discord.gg/9GDUE4hRcW)-ൽ ചേരുക.

## Installation / ഇൻസ്റ്റലേഷൻ
Either download precompiled binaries from release page or clone this repository / റിലീസ് പേജിൽ നിന്ന് കംപൈൽ ചെയ്ത ബൈനറികൾ ഡൗൺലോഡ് ചെയ്യുക അല്ലെങ്കിൽ ഈ റിപ്പോസിറ്ററി ക്ലോൺ ചെയ്യുക:

`git clone https://github.com/sreyas-sreelal/malluscript.git`

## Building / ബിൽഡിംഗ്
The interpreter can be compiled as follows / ഇൻ്റർപ്രെറ്റർ താഴെ പറയുന്ന രീതിയിൽ കംപൈൽ ചെയ്യാം:

```
cd malluscript
cargo build --release
```
Note: You require Rust compiler installed on your machine in order to compile the interpreter / ശ്രദ്ധിക്കുക: ഇൻ്റർപ്രെറ്റർ കംപൈൽ ചെയ്യുന്നതിനായി നിങ്ങളുടെ സിസ്റ്റത്തിൽ Rust കംപൈലർ ഇൻസ്റ്റാൾ ചെയ്തിരിക്കണം.

## Executing malluscript programs / പ്രോഗ്രാമുകൾ പ്രവർത്തിപ്പിക്കാൻ
Malluscript can be used in two ways / മല്ലുസ്ക്രിപ്റ്റ് രണ്ട് രീതികളിൽ ഉപയോഗിക്കാം:

### Interactive shell / ഇൻ്ററാക്ടീവ് ഷെൽ
To start the interactive shell, just run following in the terminal / ഇൻ്ററാക്ടീവ് ഷെൽ ആരംഭിക്കാൻ ടെർമിനലിൽ താഴെ പറയുന്നത് പ്രവർത്തിപ്പിക്കുക:

```
./malluscript
```
Something like this would come / താഴെ കാണുന്നതുപോലെ ദൃശ്യമാകും:
```
Mallu Script Version 0.1.0
>> 
```
Here you can start writing your malluscript codes on the way! / നിങ്ങൾക്ക് ഇവിടെ നേരിട്ട് സ്ക്രിപ്റ്റുകൾ എഴുതാൻ സാധിക്കും.

### Run malluscript program files / ഫയലുകൾ പ്രവർത്തിപ്പിക്കാൻ
Write the code in the file with `.ms` extension and execute it as follows / `.ms` എക്സ്റ്റൻഷൻ ഉള്ള ഫയലുകളിൽ കോഡ് എഴുതി താഴെ പറയുന്ന രീതിയിൽ പ്രവർത്തിപ്പിക്കാം:

```
./malluscript file_name.ms
```

## Language Syntax And Grammar / വ്യാകരണവും ചിന്താഗതിയും

Basic arithmetic operations can be done using `+`,`-`,`*`,`/`,`%` (modulo). Every expression ends with `;`. `{` and `}` introduces a new block like c like languages. Every keyword has alternate ones too, each with different dialects and also in Malayalam unicodes.

മല്ലുസ്ക്രിപ്റ്റിൽ ഗണിതക്രിയകൾക്കായി `+`,`-`,`*`,`/`,`%` എന്നിവ ഉപയോഗിക്കാം. എല്ലാ വാചകങ്ങളും `;`-ൽ അവസാനിക്കണം. സി പ്രോഗ്രാമിംഗ് ഭാഷയുടേതുപോലെ തന്നെ പുതിയ ബ്ലോക്കുകൾക്കായി `{`, `}` എന്നിവ ഉപയോഗിക്കുന്നു. മിക്ക കീവേഡുകൾക്കും ഇംഗ്ലീഷിലുള്ള ശബ്ദരൂപവും മലയാളത്തിലുള്ള അക്ഷരരൂപവും ലഭ്യമാണ്.

### Basic I/O operations / ഇൻപുട്ട് ഔട്ട്‌പുട്ട് രീതികൾ
* Printing or writing to console is done as follows / കൺസോളിൽ ഔട്ട്‌പുട്ട് കാണിക്കുന്നതിനായി താഴെ പറയുന്ന രീതി ഉപയോഗിക്കാം:
    ```ms
    "hello world" ezhuthuga;
    variable ezhuthuga;
    "Onnum randum kottiyal " + 1+2 + " aanu" ezhuthuga; 
    ``` 
  * Alternate Keywords / മറ്റ് കീവേഡുകൾ:
    * `ezhuthuka`
    * `ezhuthuga`
    * `kanikuka`
    * `kanikuga`
    * `എഴുതുക`
    * `കാണിക്കുക`

* In order to get keyboard input from the user / കീബോർഡിൽ നിന്ന് ഇൻപുട്ട് സ്വീകരിക്കാൻ:
  
  For integer inputs
  ```
  variable = akam_vangikuga;
  ```
  * Alternate Keywords
    * `akam_vangikuga`
    * `akkam_vangikuga`
    * `അക്കംവാങ്ങിക്കുക`

  For string inputs
  ```
  variable = vachakam_vangikuga;
  ```
  * Alternate Keywords
    * `vachakam_vangikuga`
    * `vachakam_vangikuka`
    * `vachagam_vangikuga`
    * `vachagam_vangikuka`
    * `വാചകംവാങ്ങിക്കുക`

### Datatypes and Storage / ഡേറ്റാ ടൈപ്പുകളും സംഭരണവും
Malluscript is not strictly type-safe. The language supports integer, float and string literals as primary datatypes and it also support compound datatype called List.

മല്ലുസ്ക്രിപ്റ്റ് ഡേറ്റാ ടൈപ്പുകളുടെ കാര്യത്തിൽ അത്ര കർക്കശമല്ല. ഇൻ്റിജർ, ഫ്ലോട്ട്, സ്ട്രിംഗ് എന്നിവ കൂടാതെ ലിസ്റ്റ് (List) എന്ന കോമ്പൗണ്ട് ഡേറ്റാ ടൈപ്പും ഇതിൽ ലഭ്യമാണ്.

To assign it some value and declare a variable / ഒരു വേരിയബിൾ ഡിക്ലയർ ചെയ്യാനായി താഴെ പറയുന്ന രീതി ഉപയോഗിക്കാം:

```
variable_name = 1;
second_var = "ente string";
```

Lists in Malluscript are similar to compound types in other mainstream programming languages. A list can have different type of primary data in it. An example usage of List would be:

```
x = [1,4,5,6,"malluscript"];
x = x + "more data";
i = 0;
i um 6 um onnallenkil avarthikuga {
    x[i] + " " ezhuthuga;
    i = i+1;
}
```
Using `+` operator along side a list data, will append the other operand into that list. List data can be accessed using subscript operators `[]`. Also unlike other datatypes when passed as an argument to a function, it will be *passed as reference* instead of *passed as value*.

## Modules and Imports / മോഡ്യൂളുകളും ഇംപോർട്ടുകളും
Malluscript supports a powerful module system for code organization. Modules are imported using the `ulppeduthuka` keyword.

മല്ലുസ്ക്രിപ്റ്റിൽ പ്രോഗ്രാമുകളെ വിവിധ ഭാഗങ്ങളായി തിരിച്ച് എഴുതാൻ സാധിക്കും. മറ്റ് ഫയലുകൾ ഉൾപ്പെടുത്താനായി `ulppeduthuka` എന്ന കീവേഡാണ് ഉപയോഗിക്കുന്നത്.

### Basic Import / ഇംപോർട്ടുകൾ
Imports the specified module from the filesystem. The module path is relative to the current working directory.
```ms
math ulppeduthuka;
math.math_add<3, 4> ezhuthuka;
```

### Module Aliasing / അപരനാമങ്ങൾ
You can alias imported modules for convenience using the `ennu` or `ayi` keywords.
```ms
math m ennu ulppeduthuka;
m.math_add<3, 4> ezhuthuka;
```
* Alternate Keywords / മറ്റ് കീവേഡുകൾ:
    * **ulppeduthuka**
      * `ulppeduthuga`
      * `ഉൾപ്പെടുത്തുക`

    * **ennu**
      * `ayi`
      * `എന്ന്`
      * `ആയി`

## Conditional Statements And Expressions / നിബന്ധനാധിഷ്ഠിത വാചകങ്ങൾ
The conditional expression has the following syntax / നിബന്ധനാധിഷ്ഠിത വാചകങ്ങളുടെ ഘടന താഴെ പറയുന്നവയാണ്:

```
i um 0 um thullyaman enkil {

} adhallengil {

}
```
The above snippet checks whether i equal to 0 and if yes the code in the first block will execute otherwise block defined by `adhallengil` will get executed.

മുകളിലെ കോഡ് `i`-യും `0`-യും തുല്യമാണോ എന്ന് പരിശോധിക്കുന്നു. അവ തുല്യമാണെങ്കിൽ ആദ്യത്തെ ബ്ലോക്കും അല്ലെങ്കിൽ `adhallengil` ബ്ലോക്കും പ്രവർത്തിക്കുന്നു.

In general / പൊതുവായി:
* `i um 0 um thullyaman enkil` checks if i is equals to 0 / i എന്നത് 0-ന് തുല്യമാണോ എന്ന് പരിശോധിക്കുന്നു.
* `i um 0 um thullyamalla enkil` checks if i not equals to 0 / i എന്നത് 0-ന് തുല്യമല്ലയോ എന്ന് പരിശോധിക്കുന്നു.
* `i nekal 0 cheruthan enkil` checks if 0 is less than i / 0 എന്നത് i-യേക്കാൾ ചെറുതാണോ എന്ന് പരിശോധിക്കുന്നു.
* `i nekal 0 veluthan enkil` checks if 0 is greater than i / 0 എന്നത് i-യേക്കാൾ വലുതാണോ എന്ന് പരിശോധിക്കുന്നു.

  * Alternate Keywords
    * **enkil**
      * `engil`
      * `എങ്കിൽ`
    * **thullyamalla**
      * `onnalla`
      * `തുല്യമല്ല`
      * `ഒന്നല്ല`
    * **thullyaman**
      * `onnan`
      * `തുല്യമാണ്`
      * `ഒന്നാണ്`
    * **um**
      * `ഉം`
    * **ne_kal**
      * `നെകാൾ`
    * **veluthan**
      * `വലുതാണ്`
    * **cheruthan**
      * `ചെറുതാണ്`
    * **adhallengil**
      * `adhallenkil`
      * `അതല്ലെങ്കിൽ`

#### Agglutination of conditional statements / സന്ധിരൂപങ്ങൾ
Conditional statements like `thullyaman enkil` can be agglutinated together like `thullyamanenkil`. / `thullyaman enkil` പോലുള്ള വാചകങ്ങൾ `thullyamanenkil` എന്ന് ഒറ്റവാക്കായും ഉപയോഗിക്കാം.

## Iterative Statements Or Loops / ആവർത്തന വാചകങ്ങൾ
The loops in malluscript look as follows / മല്ലുസ്ക്രിപ്റ്റിലെ ആവർത്തന രീതി താഴെ പറയുന്ന രീതിയിലാണ്:
```
0 nekal i veluthan enkil avarthikuga {
  i = i-1;
}
```
  * Alternative Keywords / മറ്റ് കീവേഡുകൾ:
    * `aavarthikuga`
    * `avarthikuga`
    * `aavarthikuka`
    * `avarthikuka`
    * `ആവർത്തിക്കുക`

## Functions / ഫങ്ഷനുകൾ
The functions definition in malluscript are defined as follows / ഫങ്ഷനുകൾ ഡിക്ലയർ ചെയ്യുന്നതിനായി താഴെ പറയുന്ന രീതി ഉപയോഗിക്കാം:
```
ente_function(variable1,variable2) {
  variable1 + variable2 ezhuthuga;
}
```
The functions can also return values using `kodukuga`. / വിഭവങ്ങൾ തിരികെ നൽകാനായി (Return) `kodukuga` ഉപയോഗിക്കാം:
```
factorial(n) {
    n um 0 um thullyaman enkil {
        1 kodukuga;   
    }
    n * factorial<n-1> kodukuga;
}
```
  * Alterative keywords for `kodukuga` (Return) / മറ്റ് കീവേഡുകൾ:
    * `kodukuka`
    * `madakiayakuga`
    * `madakiayakuka`
    * `കൊടുക്കുക`
    * `മടക്കിഅയയ്ക്കുക`

To call a function function name followed by angle brackets are used. / ഒരു ഫങ്ഷൻ പ്രവർത്തിപ്പിക്കാൻ അതിൻ്റെ പേരിനോടൊപ്പം ആംഗിൾ ബ്രാക്കറ്റുകൾ ഉപയോഗിക്കുക:
```
my_function<arg1,arg2>;
```

## Example / ഉദാഹരണം
A simple program to find factorial in malluscript would be / മല്ലുസ്ക്രിപ്റ്റിൽ ഫാക്ടോറിയൽ കണ്ടെത്താനുള്ള ഒരു എളുപ്പവഴി താഴെ പറയുന്നതാണ്:

```ms
"Input number:" kanikuka;
num = akam_vangikuga;
factorial = 1;

0 um num um thullyamalla enkil avarthikuga {
    factorial = factorial * num;
    num = num - 1;
}

"Factoral is : " + factorial + "\n" kanikuka;

```
Same in pure Malayalam would be / ഇതേ പ്രോഗ്രാം ശുദ്ധമലയാളത്തിൽ താഴെ പറയുന്ന രീതിയിൽ എഴുതാം:
```ms
"ദയവായി നമ്പർ തരുക: " എഴുതുക;
നമ്പർ = അക്കംവാങ്ങിക്കുക;
ഫാക്ടോറിയൽ = 1;

0 ഉം നമ്പർ ഉം തുല്യമല്ലെങ്കിൽ ആവർത്തിക്കുക {
  ഫാക്ടോറിയൽ = ഫാക്ടോറിയൽ * നമ്പർ;
  നമ്പർ = നമ്പർ - 1;
}

"ഫാക്ടോറിയൽ : " + ഫാക്ടോറിയൽ + "\n" എഴുതുക;
```


More examples can be found in [examples](examples)

## Notes / കുറിപ്പുകൾ
The language as mentioned above is under the development phase and its structure can change overnight, suddenly.

മുകളിൽ സൂചിപ്പിച്ചത് പോലെ, ഈ ഭാഷ വികസനത്തിന്റെ പ്രാഥമിക ഘട്ടത്തിലാണ്. അതിനാൽ ഇതിന്റെ ഘടനയിൽ എപ്പോൾ വേണമെങ്കിലും മാറ്റങ്ങൾ ഉണ്ടായേക്കാം.

Any kind of contribution is always welcome. If you have any ideas or improvements to provide for this project open a pull request or if you have any difficulties using this language open an issue :) 

ഈ പ്രോജക്റ്റിലേക്കുള്ള ഏത് തരത്തിലുള്ള സംഭാവനകളും സ്വാഗതം ചെയ്യുന്നു. നിങ്ങൾക്ക് എന്തെങ്കിലും ആശയങ്ങളോ മെച്ചപ്പെടുത്തലുകളോ ഉണ്ടെങ്കിൽ ഒരു പുൾ റിക്വസ്റ്റ് (Pull Request) നൽകുക. എന്തെങ്കിലും ബുദ്ധിമുട്ടുകൾ നേരിട്ടാൽ ഒരു ഇഷ്യൂ (Issue) ക്രിയേറ്റ് ചെയ്യുക :)

