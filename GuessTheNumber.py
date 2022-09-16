import random

print("Welcome to the Guess the Number game!")
print("")

number = random.randint(1,100)

guess = 0

while(guess != number):
    guess = int(input("Guess a number between 1 and 100. "))

    if (guess < number):
        print("Too small. Guess a larger number.")

    elif (guess > number):
        print("Too large. Guess a smaller number.")

    elif (guess == number):
        print("Correct! You win!")