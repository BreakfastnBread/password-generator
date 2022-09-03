#simple password generator, simply input a length of choice and recieve a random assortment of numbers, letters and symbols.
import random
import string
length = int(input("\nenter length of password: "))                      
lower = string.ascii_lowercase
upper = string.ascii_uppercase
num = string.digits
symbols = string.punctuation
all = lower + upper + num + symbols
temp = random.sample(all,length)
password = "".join(temp)
print(password)