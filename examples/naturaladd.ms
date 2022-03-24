pwoli_sadhanam n;
pwoli_sadhanam i;
pwoli_sadhanam sum;
i=1;
sum=0;
dhe_pidicho "Enter the limit: ";
n = number_thada;
repeat_adi n ne_kal i cheruthane {
    sum = sum + i;
    i = i + 1;
}
sum=sum+n;
dhe_pidicho "The sum of natural numbers upto "+n+" is "+sum+"\n";
