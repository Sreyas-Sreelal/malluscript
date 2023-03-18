pow(a,t)
{
    pwoli_sadhanam w;
    w=1;
    repeat_adi 0 um t um same_alle
    {
        w=w*a;
        t=t-1;
    }
    koduthek w;
}
pwoli_sadhanam n;
dhe_pidicho "Enter number";
n=number_thada;
pwoli_sadhanam k;
k=n;
pwoli_sadhanam l;
pwoli_sadhanam m;
pwoli_sadhanam p;
pwoli_sadhanam q;
q=n;
repeat_adi 0 um k um same_alle
{
    l=l+1;
    k=k/10;
}
repeat_adi 0 um n um same_alle
{
    m=n&10;
    p=p+pow<m,l>;
    n=n/10;
}
seriyano_mwone p um q um same_aane
{
    dhe_pidicho "Armstrong number\n";
}
seri_allel
{
    dhe_pidicho "Not an Armstrong number\n";
}
