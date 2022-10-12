let a = "global";
{
    function showA() {
        console.log(a);
    }
    
    showA();
    let a = "block";
    showA();
}
