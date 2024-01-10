kanikuga "Enter number: ";
number = akam_vangikuga;
i = 2;
flag = 1;

i ne_kal (number/2)+1 veluthan enkil avarthikuga {
    number % i um 0 um thullyaman enkil {
        flag = 0;
    }
    i = i+1;
}

flag um 0 um thullyaman enkil {
    kanikuga "Prime alle";
} adhallengil {
    kanikuga "Prime aane";
}


