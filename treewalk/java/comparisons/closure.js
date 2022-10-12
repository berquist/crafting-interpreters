var a = "global";
{
    function showA() {
        console.log(a);
    }
    
    showA();
    var a = "block";
    showA();
}
