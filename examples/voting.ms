pwoli_sadhanam age;
dhe_pidicho "Input age: ";
age=number_thada;
seriyano age um 18 um same_aane
{
    dhe_pidicho "You are eligible to vote.\n";
}
seri_allel
{
    seriyano 18 ne_kal age veluthane
    {
        dhe_pidicho "You are eligible to vote.\n";
    }
    seri_allel
    {
        dhe_pidicho "You are not eligible to vote.\n";
    }
}
