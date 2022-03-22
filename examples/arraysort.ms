pwoli_sadhanam i;
pwoli_sadhanam j;
pwoli_sadhanam n;
i=0;
j=0;
dhe_pidicho "Enter the number of elements in the list: ";
n = number_thada;
pwoli_sadhanam a[n];
dhe_pidicho "Enter the elements of the list: ";
repeat_adi n nekal i cheruthane {
    a[i] = number_thada;
    i = i + 1;
}
i=0;
j=i+1;
repeat_adi n nekal i cheruthane 
{
    repeat_adi n nekal j cheruthane 
    {
        seriyano a[j] nekal a[i] veluthane
        {
            pwoli_sadhanam temp;
            temp = a[i];
            a[i] = a[j];
            a[j] = temp;
        }
        j = j + 1;
    }
    i = i + 1;
}
