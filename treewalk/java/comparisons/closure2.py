a = "global"

with open("/tmp/hi", "w") as _:
    def showA():
        print(a)

    showA()
    a = "block"
    showA()
