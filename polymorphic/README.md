# Polymorphic Malware

## What it does

For this first prototype, we developed a program able to run a harmless payload.
It will do the following:
1. Decrypt the part of the code marked as toxic (the payload). For this example, we used XOR
   encryption using a key embedded in the binary.
2. Execute the payload (print the current time).
3. Create a new random key.
4. Encrypt the payload using the new key and overwrite the toxic section in the binary.
5. Overwrite the new key in the binary.

## How it works

A few question might arise from those explanations:
- **How can someone mark a section of the code as toxic ?**

  Using attributes, we can make sure that elements of the code (functions, variables) can be
  placed at a specific location in the binary during the program build process.


- **What does it mean for the XOR key to be embedded in the binary ?**

  Nothing very complicated ! The key is also placed in a dedicated section of the code named `.key`
  whereas the payload in stored in the `.toxic` one.
  So, the same way that the payload function code can be overwritten, the key variable can be 
  changed.


- **Are there any kind of protections against this technique ?**

  One unexpected protection we faced and could not overcome during the development was specific 
  to macOS platforms. You see, macOS refuses to execute code that might set specific sections of 
  the code writeable, something required to overwrite the payload's code.
  
  Using more usual processes, we can reverse engineer the binary to inspect its structure and 
  behaviour, try to run it in a sandbox environment and check the before and after binary's 
  content, maybe use machine learning...