import os.path
import os
import sys
import subprocess
from gpiozero import LED
from time import sleep
import RPi.GPIO as GPIO
import telepot
from telepot.loop import MessageLoop

#Start all Sensors
processes = []

# TELEGRAM BOT 
#Variable to manage the status of Home SecuriPi System
status = False

#Function that take in input the Telegram bot message and handle it
def handle(msg):
    global status
    chat_id = msg['chat']['id']
    command = msg['text']
    #List of commands usable in Telegram
        #Command that activate Home SecuriPi System
    if command == '/arm': 
        p = subprocess.Popen([sys.executable,'mic.py'],shell=True, stdout = subprocess.PIPE, stderr = subprocess.STDOUT)
        processes.append(p)
        p = subprocess.Popen([sys.executable,'camera.py'],shell=True, stdout = subprocess.PIPE, stderr = subprocess.STDOUT)
        processes.append(p)
        p = subprocess.Popen([sys.executable,'pir.py'],shell=True, stdout = subprocess.PIPE, stderr = subprocess.STDOUT)
        processes.append(p)
        if status == True:
            bot.sendMessage(chat_id, "Home SecuriPi already Activated")
        else:
            status = True
            bot.sendMessage(chat_id, "Home SecuriPi Activated")
        #Command that deactivate Home SecuriPi System
    elif command == '/disarm':
        for x in processes:
            x.terminate()
        if status == False:
            bot.sendMessage(chat_id, "Home SecuriPi already Deactivated")
        else:
            status = False
            bot.sendMessage(chat_id, "Home SecuriPi Deactivated")
        #Command for Copyright
    elif command == '/jst':
        bot.sendMessage(chat_id, "JST RULEZ")
        #Command to receive status of Home SecuriPi System
    elif command == '/status':
        if status == True: 
            bot.sendMessage(chat_id, "Home SecuriPi Status Activated")
        else:
            bot.sendMessage(chat_id, "Home SecuriPi Status Deactivated")
        #Command to receive info of Home SecuriPi System
    elif command == '/info':
        bot.sendMessage(chat_id, "Info: Group JST Mechatronics Project, created by Matteo Guerrini, Giacomo Menchi and William Paciaroni. To start using the bot write / to show the list of commands")

#Telegram bot: defining Token for request API, declaring the bot to send/receive messages in another thread
token = ''
bot = telepot.Bot(token)
MessageLoop(bot, handle).run_as_thread()
bot.sendMessage(chat_id="-480745285", text="Home SecuriPi System Online")


# CORE HOME SECURITY SYSTEM
#Set up GPIO of Raspberry
GPIO.setwarnings(False)
GPIO.setmode(GPIO.BCM)
GPIO.setup(18, GPIO.OUT)  
p = GPIO.PWM(18, 50)

#Start CORE of Home SecuriPi, that check if the txt file exists and the status is on (by Telegram bot)
#Then start the Speaker and send the notification to the Telegram bot
while True:
    val = os.path.exists('/home/checker.txt')
    if val==True and status==True:
        bot.sendMessage(chat_id="-480745285", text="ALARM! Movements detected in your place")
        for i in range(50):
            p.start(20)
        for x in range(200, 2200):
            p.ChangeFrequency(x)
            sleep(0.0001)
        p.stop()  
        os.remove('/home/checker.txt')