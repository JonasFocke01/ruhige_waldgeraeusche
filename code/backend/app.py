# run 'pip install -U Flask cd \py-script' before 
# and 'pip install flask-cors
# to run the app: 'python -m flask run --host 192.168.2.17'

from flask import Flask, request
from flask_cors import CORS
from logic.controller import *
from logic.led_strips import *

app = Flask(__name__)
CORS(app)

@app.route("/")
def http_start():
    start_controller()
    return "Test passed!\nRest api is online!"

# leds
@app.route('/master_level/',methods = ['POST'])
def http_master_level():
    master_level(request.json['value'])
    return 'changed master level of leds'

# TODO
@app.route("/toggle_beat/")
def http_toggle_beat():
    toggle_beat()
    return "beat toggled"

@app.route('/color_mode_white/')
def http_color_mode_white():
    color_mode_white()
    return 'color changed to white'

@app.route('/color_mode_inherit/')
def http_color_mode_inherit():
    color_mode_inherit()
    return 'color changed to inherit color'

@app.route('/switch_to_group_movement/')
def http_all_as_one():
    switch_to_group_movement()
    return 'changed to act all as one'

@app.route('/switch_to_individual_movement/')
def http_individualy():
    switch_to_individual_movement()
    return 'changed to act all as individuals'

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
    return 'flashed leds'

@app.route('/flash_fade/')
def http_flash_fade():
    flash_fade()
    return 'flashed leds and faded them out'

@app.route('/animation_trigger/')
def http_animation_trigger():
    animation_trigger()
    return 'triggered animation'

@app.route('/toggle_active_animation_off/')
def http_toggle_active_animation_off():
    toggle_active_animation_off()
    return 'changed to display no animation'

@app.route('/clear_strips/')
def http_clear_strips():
    clear_strips()
    return 'cleared strips'

# TODO: refactor
@app.route('/animation_snake/')
def http_animation_snake():
    animation_snake()
    return 'switched animation to snake'

# TODO: refactor
@app.route('/animation_rain_drops/')
def http_animation_raindrop():
    animation_rain_drops()
    return 'switched animation to raindrop'

# color panel
@app.route('/change_colors/',methods = ['POST'])
def http_change_colors():
    change_rendercolors(request.json['red'], request.json['green'], request.json['blue'])
    return 'colors changed'