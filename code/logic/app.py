# run 'pip install -U Flask cd \py-script' before 
# and 'pip install flask-cors
# to run the app: 'python -m flask run --host 192.168.2.17'

from flask import Flask, request
from flask_cors import CORS
import os
from controller import *
from led_strips import *

app = Flask(__name__)
CORS(app)

@app.route("/")
def http_start():
    start_controller()
    return "Test passed!\nRest api is online!"

# leds
@app.route('/master_level/')
def http_master_level():
    master_level()
    return 'changed master level of leds'

@app.route("/toggle_beat/")
def http_toggle_beat():
    toggle_beat()
    return "beat toggled"

@app.route('/color_white/')
def http_color_white():
    color_white()
    return 'color changed to white'

@app.route('/color_color/')
def http_color_color():
    color_color()
    return 'color changed to inherit color'

@app.route('/all_as_one/')
def http_all_as_one():
    all_as_one()
    return 'changed to act all as one'

@app.route('/individualy/')
def http_individualy():
    individualy()
    return 'changed to act all as individuence'

@app.route('/animation_off/')
def http_animation_off():
    animation_off()
    return 'changed to display no animation'

@app.route('/toggle_move_up/')
def http_toggle_move_up():
    toggle_move_up()
    return 'toggled moving up'

@app.route('/toggle_move_down/')
def http_toggle_move_down():
    toggle_move_down()
    return 'toggled moving down'

@app.route('/toggle_move_sideways/')
def http_toggle_move_sideways():
    toggle_move_sideways()
    return 'toggled moving sideways'

@app.route('/short_flash/')
def http_short_flash():
    short_flash()
    return 'flashed leds shortly'

@app.route('/flash_fade/')
def http_flash_fade():
    flash_fade()
    return 'flashed leds and faded them out'

@app.route('/animation_trigger/')
def http_animation_trigger():
    animation_trigger()
    return 'triggered animation'

@app.route('/animation_snake/')
def http_animation_snake():
    animation_snake()
    return 'switched animation to snake'

@app.route('/animation_raindrop/')
def http_animation_raindrop():
    animation_raindrop()
    return 'switched animation to raindrop'

@app.route('/animation_level/')
def http_animation_level():
    animation_level()
    return 'switched animation to level'

# color panel
@app.route('/change_colors/',methods = ['POST'])
def http_change_colors():
    change_rendercolors(request.json['red'], request.json['green'], request.json['blue'])
    return 'colors changed'