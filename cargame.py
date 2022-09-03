command5 = ""
command4 = ""
command3 = ""
command2 = ""
command1 = ""
while command1.lower() != "quit":
    command1 = input("> ")
    
    if command2.lower() == "help":
        print("start: starts car\nstop: stops car\nquit: ends code")
    
    elif command3.lower() == "start":
        print("Car has started!")
        if command4.lower() == "stop":
            print("the car has successfully stopped")
    
    else:
        print("Command not understood, try typing help")
