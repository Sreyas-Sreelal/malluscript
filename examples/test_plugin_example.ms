test_plugin p ennu ulppeduthuka;

"Adding 5 and 7 using native plugin: " ezhuthuka;
p.add<5, 7> ezhuthuka;
"\n" ezhuthuka;

"Concatenating strings using native plugin: " ezhuthuka;
p.concat<"Hello, ", "World!"> ezhuthuka;
"\n" ezhuthuka;

"Reversing list [1, 2, 3] using native plugin: " ezhuthuka;
my_list = [1, 2, 3];
p.reverse_list<my_list> ezhuthuka;
"\n" ezhuthuka;

"Testing cross-boundary native call (invoke_malluscript -> add): " ezhuthuka;
result = p.invoke_malluscript<"add", 50, 10>;
result ezhuthuka;
"\n" ezhuthuka;

