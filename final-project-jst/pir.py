from gpiozero import MotionSensor
import os.path

pir = MotionSensor(16)

def createFile():
    return open("checker.txt",'w+')

while True:
    pir.wait_for_motion()
    f = createFile()
    f.close()