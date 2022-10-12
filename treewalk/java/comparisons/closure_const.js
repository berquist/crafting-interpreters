const a = "global";
{
    function showA() {
        console.log(a);
    }
    
    showA();
    const a = "block";
    showA();
}
