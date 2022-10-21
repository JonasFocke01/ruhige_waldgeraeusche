# run 'pip install -U Flask cd \py-script' before 
# to run the app: 'python -m flask run --host 192.168.2.17'

from flask import Flask
import os
from controller import *
from led_strips import *

app = Flask(__name__)

@app.route("/")
def start():
    start_controller()
    return "Hello World"

@app.route("/snake/")
def snake():
    spawn_snake()
    return "snake spawned"

@app.route("/color/")
def color():
    change_rendercolors(random.randrange(0, 255), random.randrange(0, 255), random.randrange(0, 255))
    return "color changed"
