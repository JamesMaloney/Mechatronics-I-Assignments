import RPi.GPIO as GPIO
import time

#GPIO SETUP
channel = 27
GPIO.setmode(GPIO.BCM)
GPIO.setup(channel, GPIO.IN)

def createFile():
    return open("checker.txt",'w+')

def callback(channel):
        if GPIO.input(channel):
            print "No signal"
        else:
            f = createFile()
            f.close()

GPIO.add_event_detect(channel, GPIO.BOTH, bouncetime=300)  # let us know when the pin goes HIGH or LOW
GPIO.add_event_callback(channel, callback)  # assign function to GPIO PIN, Run function on change

# infinite loop
while True:
    time.sleep(1)